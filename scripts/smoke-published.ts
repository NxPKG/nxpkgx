#!/usr/bin/env node

import { execSync, ExecSyncOptions } from "child_process";

type PackageManager = "yarn" | "npm" | "pnpm";
type InstallType = "local" | "global";
type VersionInput = {
  version: string;
  type: InstallType;
};
type Condition = {
  expected: string;
  operator: "includes" | "notIncludes" | "startsWith";
};

type TestInput = {
  local: VersionInput;
  global: VersionInput;
  packageManager: PackageManager;
};

type Args = [keyof typeof tests, PackageManager, string, string];

function exec({
  command,
  title,
  options,
  conditions,
}: {
  command: string;
  title?: string;
  options?: ExecSyncOptions;
  conditions?: Array<Condition>;
}) {
  console.log();
  if (title) {
    console.log(`ℹ️ ${title}`);
  }
  console.log(`Running: "${command}"`);
  try {
    const result = execSync(command, options).toString().trim();
    if (process.env.GITHUB_ACTIONS === "true") {
      console.log(`::group::"${command}" output`);
      console.log(result);
      console.log(`::endgroup::`);
    } else {
      console.log(result);
    }

    if (conditions && conditions.length > 0) {
      conditions.forEach((condition) => {
        assertOutput({ output: result, command, condition });
      });
    } else {
      return result;
    }
  } catch (err) {
    let message = "Unknown error";
    if (err instanceof Error) {
      message = err.message;
    }
    console.error(err);
    console.error(message);
    process.exit(1);
  }
}

function getGlobalBinaryPath({
  packageManager,
}: {
  packageManager: PackageManager;
}) {
  switch (packageManager) {
    case "yarn":
      return "/yarn/global/node_modules/";
    case "npm":
      return "/usr/local/lib/node_modules";
    case "pnpm":
      return "/pnpm/global/";
  }
}

function assertOutput({
  output,
  command,
  condition,
}: {
  output: string;
  command: string;
  condition: Condition;
}) {
  const { operator, expected } = condition;
  if (operator === "includes") {
    if (output.includes(expected)) {
      console.log(`✅ "${command}" output includes "${expected}"`);
    } else {
      console.error(`❌ "${command}" output does not include "${expected}"`);
      process.exit(1);
    }
  }

  if (operator === "notIncludes") {
    if (!output.includes(expected)) {
      console.log(`✅ "${command}" output does not include "${expected}"`);
    } else {
      console.error(`❌ "${command}" output does not include "${expected}"`);
      process.exit(1);
    }
  }

  if (operator === "startsWith") {
    if (output.startsWith(expected)) {
      console.log(`✅ "${command}" output starts with "${expected}"`);
    } else {
      console.error(`❌ "${command}" output does not start with "${expected}"`);
      process.exit(1);
    }
  }
}

function installExample({
  version,
  packageManager,
}: {
  version: string;
  packageManager: PackageManager;
}) {
  exec({
    title: "Install example",
    command: `npx create-nxpkg@${version} . ${packageManager}`,
    conditions: [
      {
        expected: "Success! Your new Nxpkgrepo is ready.",
        operator: "includes",
      },
    ],
  });
  if (version !== "latest" && version !== "canary") {
    exec({
      title: "Install exact nxpkg version",
      command: `${packageManager} install nxpkg@${version} --save-dev`,
    });
  }
}

function installGlobalNxpkg({
  version,
  packageManager,
}: {
  version: string;
  packageManager: PackageManager;
}) {
  if (packageManager === "pnpm" || packageManager === "npm") {
    exec({
      title: "Install global nxpkg",
      command: `${packageManager} install nxpkg@${version} --global`,
    });
  } else {
    exec({
      title: "Install global nxpkg",
      command: `${packageManager} global add nxpkg@${version}`,
    });
  }
}

function uninstallLocalNxpkg({
  packageManager,
}: {
  packageManager: PackageManager;
}) {
  if (packageManager === "pnpm" || packageManager === "npm") {
    exec({
      title: "Uninstall local nxpkg",
      command: `${packageManager} uninstall nxpkg`,
    });
  } else {
    exec({
      title: "Uninstall local nxpkg",
      command: `${packageManager} remove nxpkg -W`,
    });
  }
}

function getNxpkgBinary({
  installType,
  packageManager,
}: {
  installType: InstallType;
  packageManager: PackageManager;
}) {
  if (installType === "global") {
    return "nxpkg";
  } else {
    if (packageManager === "npm") {
      return "./node_modules/.bin/nxpkg";
    } else {
      return `${packageManager} nxpkg`;
    }
  }
}

function logNxpkgDetails({
  installType,
  packageManager,
}: {
  installType: InstallType;
  packageManager: PackageManager;
}) {
  const nxpkgBinary = getNxpkgBinary({ installType, packageManager });
  exec({ command: `${nxpkgBinary} --version` });
  exec({ command: `${nxpkgBinary} bin` });
}

function verifyLocalBinary({
  installType,
  packageManager,
}: {
  installType: InstallType;
  packageManager: PackageManager;
}) {
  const nxpkgBinary = getNxpkgBinary({ installType, packageManager });
  exec({
    title: "Verify binary is not installed globally",
    command: `${nxpkgBinary} bin`,
    conditions: [
      {
        expected:
          packageManager === "npm" ? "/usr/local/lib/node_modules" : "global",
        operator: "notIncludes",
      },
    ],
  });
}

function verifyGlobalBinary({
  installType,
  packageManager,
}: {
  installType: InstallType;
  packageManager: PackageManager;
}) {
  const packageManagerGlobalBinPath = getGlobalBinaryPath({ packageManager });
  const nxpkgBinary = getNxpkgBinary({ installType, packageManager });
  exec({
    title: "Verify binary is installed globally",
    command: `${nxpkgBinary} bin`,
    conditions: [
      {
        expected: packageManagerGlobalBinPath,
        operator: "includes",
      },
    ],
  });
}

function verifyFirstBuild({
  installType,
  packageManager,
}: {
  installType: InstallType;
  packageManager: PackageManager;
}) {
  const nxpkgBinary = getNxpkgBinary({ installType, packageManager });
  exec({
    title: "Verify first nxpkg build is successful and not cached",
    command: `${nxpkgBinary} build`,
    conditions: [
      { expected: "2 successful, 2 total", operator: "includes" },
      { expected: "0 cached, 2 total", operator: "includes" },
      { expected: "FULL NXPKG", operator: "notIncludes" },
    ],
  });
}

function verifySecondBuild({
  installType,
  packageManager,
}: {
  installType: InstallType;
  packageManager: PackageManager;
}) {
  const nxpkgBinary = getNxpkgBinary({ installType, packageManager });
  exec({
    title: "Verify second nxpkg build is successful and cached",
    command: `${nxpkgBinary} build`,
    conditions: [
      { expected: "2 successful, 2 total", operator: "includes" },
      { expected: "2 cached, 2 total", operator: "includes" },
      { expected: "FULL NXPKG", operator: "includes" },
    ],
  });
}

function local({ local, packageManager }: TestInput) {
  // setup example
  installExample({ version: local.version, packageManager });
  verifyLocalBinary({ installType: "local", packageManager });
  logNxpkgDetails({ installType: "local", packageManager });

  // verify build is correctly cached
  verifyFirstBuild({ installType: "local", packageManager });
  verifySecondBuild({ installType: "local", packageManager });
}

function global({ local, global, packageManager }: TestInput) {
  // setup example
  installExample({ version: local.version, packageManager });
  installGlobalNxpkg({ version: global.version, packageManager });
  logNxpkgDetails({ installType: "global", packageManager });

  verifyLocalBinary({ installType: "global", packageManager });
  uninstallLocalNxpkg({ packageManager });
  logNxpkgDetails({ installType: "global", packageManager });
  verifyGlobalBinary({ installType: "global", packageManager });

  // verify build is correctly cached
  verifyFirstBuild({ installType: "global", packageManager });
  verifySecondBuild({ installType: "global", packageManager });
}

function both({ local, global, packageManager }: TestInput) {
  // setup example
  installExample({ version: local.version, packageManager });
  installGlobalNxpkg({ version: global.version, packageManager });
  logNxpkgDetails({ installType: "global", packageManager });
  verifyLocalBinary({ installType: "global", packageManager });

  // verify build is correctly cached
  verifyFirstBuild({ installType: "global", packageManager });
  verifySecondBuild({ installType: "global", packageManager });
}

const tests = {
  local,
  global,
  both,
};

/**
 * Triggered via github workflow manually, or on publish of a new version
 *
 * https://github.com/nxpkg/nxpkg/blob/main/.github/workflows/nxpkgrepo-smoke-published.yml
 */
function test() {
  const args = process.argv.slice(2) as Args;
  const [
    testName = "local",
    packageManager = "pnpm",
    localVersion = "canary",
    globalVersion = "canary",
  ] = args;

  const local: VersionInput = {
    type: "local",
    version: localVersion,
  };
  const global: VersionInput = {
    type: "global",
    version: globalVersion,
  };

  console.log(
    `Running test: "${testName}" with local version: "nxpkg@${localVersion}" and global version: nxpkg@${globalVersion} using ${packageManager}`
  );
  tests[testName]({ local, global, packageManager });
  console.log("Tests passed!");
}

test();
