import * as nxpkgWorkspaces from "@turbo/workspaces";
import * as nxpkgUtils from "@turbo/utils";
import { setupTestFixtures } from "@turbo/test-utils";
import { getNxpkgUpgradeCommand } from "../src/commands/migrate/steps/getNxpkgUpgradeCommand";
import * as utils from "../src/commands/migrate/utils";
import { getWorkspaceDetailsMockReturnValue } from "./test-utils";

jest.mock("@turbo/workspaces", () => ({
  __esModule: true,
  ...jest.requireActual("@turbo/workspaces"),
}));

interface TestCase {
  version: string;
  packageManager: nxpkgUtils.PackageManager;
  packageManagerVersion: string;
  fixture: string;
  expected: string;
}

const LOCAL_INSTALL_COMMANDS: Array<TestCase> = [
  // npm - workspaces
  {
    version: "latest",
    packageManager: "npm",
    packageManagerVersion: "7.0.0",
    fixture: "normal-workspaces-dev-install",
    expected: "npm install nxpkg@latest --save-dev",
  },
  {
    version: "1.6.3",
    packageManager: "npm",
    packageManagerVersion: "7.0.0",
    fixture: "normal-workspaces-dev-install",
    expected: "npm install nxpkg@1.6.3 --save-dev",
  },
  {
    version: "canary",
    packageManager: "npm",
    packageManagerVersion: "7.0.0",
    fixture: "normal-workspaces-dev-install",
    expected: "npm install nxpkg@canary --save-dev",
  },
  {
    version: "latest",
    packageManager: "npm",
    packageManagerVersion: "7.0.0",
    fixture: "normal-workspaces",
    expected: "npm install nxpkg@latest",
  },
  // npm - single package
  {
    version: "latest",
    packageManager: "npm",
    packageManagerVersion: "7.0.0",
    fixture: "single-package-dev-install",
    expected: "npm install nxpkg@latest --save-dev",
  },
  {
    version: "latest",
    packageManager: "npm",
    packageManagerVersion: "7.0.0",
    fixture: "single-package",
    expected: "npm install nxpkg@latest",
  },
  // pnpm - workspaces
  {
    version: "latest",
    packageManager: "pnpm",
    packageManagerVersion: "7.0.0",
    fixture: "pnpm-workspaces-dev-install",
    expected: "pnpm add nxpkg@latest --save-dev -w",
  },
  {
    version: "1.6.3",
    packageManager: "pnpm",
    packageManagerVersion: "7.0.0",
    fixture: "pnpm-workspaces-dev-install",
    expected: "pnpm add nxpkg@1.6.3 --save-dev -w",
  },
  {
    version: "canary",
    packageManager: "pnpm",
    packageManagerVersion: "7.0.0",
    fixture: "pnpm-workspaces-dev-install",
    expected: "pnpm add nxpkg@canary --save-dev -w",
  },
  {
    version: "latest",
    packageManager: "pnpm",
    packageManagerVersion: "7.0.0",
    fixture: "pnpm-workspaces",
    expected: "pnpm add nxpkg@latest -w",
  },
  // pnpm - single package
  {
    version: "latest",
    packageManager: "pnpm",
    packageManagerVersion: "7.0.0",
    fixture: "single-package-dev-install",
    expected: "pnpm add nxpkg@latest --save-dev",
  },
  {
    version: "latest",
    packageManager: "pnpm",
    packageManagerVersion: "7.0.0",
    fixture: "single-package",
    expected: "pnpm add nxpkg@latest",
  },
  // yarn 1.x - workspaces
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "1.22.19",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn add nxpkg@latest --dev -W",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "1.22.19",
    fixture: "normal-workspaces",
    expected: "yarn add nxpkg@latest -W",
  },
  {
    version: "1.6.3",
    packageManager: "yarn",
    packageManagerVersion: "1.22.19",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn add nxpkg@1.6.3 --dev -W",
  },
  {
    version: "canary",
    packageManager: "yarn",
    packageManagerVersion: "1.22.19",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn add nxpkg@canary --dev -W",
  },
  // yarn 1.x - single package
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "1.22.19",
    fixture: "single-package-dev-install",
    expected: "yarn add nxpkg@latest --dev",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "1.22.19",
    fixture: "single-package",
    expected: "yarn add nxpkg@latest",
  },
  // yarn 2.x - workspaces
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "2.3.4",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn add nxpkg@latest --dev",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "2.3.4",
    fixture: "normal-workspaces",
    expected: "yarn add nxpkg@latest",
  },
  {
    version: "1.6.3",
    packageManager: "yarn",
    packageManagerVersion: "2.3.4",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn add nxpkg@1.6.3 --dev",
  },
  {
    version: "canary",
    packageManager: "yarn",
    packageManagerVersion: "2.3.4",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn add nxpkg@canary --dev",
  },
  // yarn 2.x - single package
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "2.3.4",
    fixture: "single-package-dev-install",
    expected: "yarn add nxpkg@latest --dev",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "2.3.4",
    fixture: "single-package",
    expected: "yarn add nxpkg@latest",
  },
  // yarn 3.x - workspaces
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "3.3.4",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn add nxpkg@latest --dev",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "3.3.4",
    fixture: "normal-workspaces",
    expected: "yarn add nxpkg@latest",
  },
  {
    version: "1.6.3",
    packageManager: "yarn",
    packageManagerVersion: "3.3.4",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn add nxpkg@1.6.3 --dev",
  },
  {
    version: "canary",
    packageManager: "yarn",
    packageManagerVersion: "3.3.4",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn add nxpkg@canary --dev",
  },
  // yarn 3.x - single package
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "3.3.4",
    fixture: "single-package-dev-install",
    expected: "yarn add nxpkg@latest --dev",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "3.3.4",
    fixture: "single-package",
    expected: "yarn add nxpkg@latest",
  },
];

const GLOBAL_INSTALL_COMMANDS: Array<TestCase> = [
  // npm
  {
    version: "latest",
    packageManager: "npm",
    packageManagerVersion: "7.0.0",
    fixture: "normal-workspaces-dev-install",
    expected: "npm install nxpkg@latest --global",
  },
  {
    version: "1.6.3",
    packageManager: "npm",
    packageManagerVersion: "7.0.0",
    fixture: "normal-workspaces-dev-install",
    expected: "npm install nxpkg@1.6.3 --global",
  },
  {
    version: "latest",
    packageManager: "npm",
    packageManagerVersion: "7.0.0",
    fixture: "normal-workspaces",
    expected: "npm install nxpkg@latest --global",
  },
  {
    version: "latest",
    packageManager: "npm",
    packageManagerVersion: "7.0.0",
    fixture: "single-package",
    expected: "npm install nxpkg@latest --global",
  },
  {
    version: "latest",
    packageManager: "npm",
    packageManagerVersion: "7.0.0",
    fixture: "single-package-dev-install",
    expected: "npm install nxpkg@latest --global",
  },
  // pnpm
  {
    version: "latest",
    packageManager: "pnpm",
    packageManagerVersion: "7.0.0",
    fixture: "pnpm-workspaces-dev-install",
    expected: "pnpm add nxpkg@latest --global",
  },
  {
    version: "1.6.3",
    packageManager: "pnpm",
    packageManagerVersion: "7.0.0",
    fixture: "pnpm-workspaces-dev-install",
    expected: "pnpm add nxpkg@1.6.3 --global",
  },
  {
    version: "latest",
    packageManager: "pnpm",
    packageManagerVersion: "7.0.0",
    fixture: "pnpm-workspaces",
    expected: "pnpm add nxpkg@latest --global",
  },
  {
    version: "latest",
    packageManager: "pnpm",
    packageManagerVersion: "7.0.0",
    fixture: "single-package",
    expected: "pnpm add nxpkg@latest --global",
  },
  {
    version: "latest",
    packageManager: "pnpm",
    packageManagerVersion: "7.0.0",
    fixture: "single-package-dev-install",
    expected: "pnpm add nxpkg@latest --global",
  },
  // yarn 1.x
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "1.22.19",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn global add nxpkg@latest",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "1.22.19",
    fixture: "normal-workspaces",
    expected: "yarn global add nxpkg@latest",
  },
  {
    version: "1.6.3",
    packageManager: "yarn",
    packageManagerVersion: "1.22.19",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn global add nxpkg@1.6.3",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "1.22.19",
    fixture: "single-package",
    expected: "yarn global add nxpkg@latest",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "1.22.19",
    fixture: "single-package-dev-install",
    expected: "yarn global add nxpkg@latest",
  },
  // yarn 2.x
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "2.3.4",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn global add nxpkg@latest",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "2.3.4",
    fixture: "normal-workspaces",
    expected: "yarn global add nxpkg@latest",
  },
  {
    version: "1.6.3",
    packageManager: "yarn",
    packageManagerVersion: "2.3.4",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn global add nxpkg@1.6.3",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "2.3.4",
    fixture: "single-package",
    expected: "yarn global add nxpkg@latest",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "2.3.4",
    fixture: "single-package-dev-install",
    expected: "yarn global add nxpkg@latest",
  },
  // yarn 3.x
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "3.3.3",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn global add nxpkg@latest",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "3.3.3",
    fixture: "normal-workspaces",
    expected: "yarn global add nxpkg@latest",
  },
  {
    version: "1.6.3",
    packageManager: "yarn",
    packageManagerVersion: "3.3.3",
    fixture: "normal-workspaces-dev-install",
    expected: "yarn global add nxpkg@1.6.3",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "3.3.4",
    fixture: "single-package",
    expected: "yarn global add nxpkg@latest",
  },
  {
    version: "latest",
    packageManager: "yarn",
    packageManagerVersion: "3.3.4",
    fixture: "single-package-dev-install",
    expected: "yarn global add nxpkg@latest",
  },
];

describe("get-nxpkg-upgrade-command", () => {
  const { useFixture } = setupTestFixtures({
    directory: __dirname,
    test: "get-nxpkg-upgrade-command",
  });

  test.each(LOCAL_INSTALL_COMMANDS)(
    "returns correct upgrade command for local install of nxpkg@$version using $packageManager@$packageManagerVersion (fixture: $fixture)",
    async ({
      version,
      packageManager,
      packageManagerVersion,
      fixture,
      expected,
    }) => {
      const { root } = useFixture({
        fixture,
      });

      const mockedExec = jest
        .spyOn(utils, "exec")
        .mockImplementation((command: string) => {
          // fail the check for global nxpkg
          if (command.includes("bin")) {
            return undefined;
          }
        });
      const mockGetPackageManagersBinPaths = jest
        .spyOn(nxpkgUtils, "getPackageManagersBinPaths")
        .mockResolvedValue({
          pnpm: undefined,
          npm: undefined,
          yarn: undefined,
          bun: undefined,
        });
      const mockGetAvailablePackageManagers = jest
        .spyOn(nxpkgUtils, "getAvailablePackageManagers")
        .mockResolvedValue({
          pnpm: packageManager === "pnpm" ? packageManagerVersion : undefined,
          npm: packageManager === "npm" ? packageManagerVersion : undefined,
          yarn: packageManager === "yarn" ? packageManagerVersion : undefined,
          bun: packageManager === "bun" ? packageManagerVersion : undefined,
        });

      const project = getWorkspaceDetailsMockReturnValue({
        root,
        packageManager,
        singlePackage: fixture.includes("single-package"),
      });
      const mockGetWorkspaceDetails = jest
        .spyOn(nxpkgWorkspaces, "getWorkspaceDetails")
        .mockResolvedValue(project);

      // get the command
      const upgradeCommand = await getNxpkgUpgradeCommand({
        project,
        to: version === "latest" ? undefined : version,
      });

      expect(upgradeCommand).toEqual(expected);

      mockedExec.mockRestore();
      mockGetPackageManagersBinPaths.mockRestore();
      mockGetAvailablePackageManagers.mockRestore();
      mockGetWorkspaceDetails.mockRestore();
    }
  );

  test.each(GLOBAL_INSTALL_COMMANDS)(
    "returns correct upgrade command for global install of nxpkg@$version using $packageManager@$packageManagerVersion (fixture: $fixture)",
    async ({
      version,
      packageManager,
      packageManagerVersion,
      fixture,
      expected,
    }) => {
      const { root } = useFixture({
        fixture,
      });

      const mockedExec = jest
        .spyOn(utils, "exec")
        .mockImplementation((command: string) => {
          if (command === "nxpkg bin") {
            return `/global/${packageManager}/bin/nxpkg`;
          }
          return undefined;
        });
      const mockGetPackageManagersBinPaths = jest
        .spyOn(nxpkgUtils, "getPackageManagersBinPaths")
        .mockResolvedValue({
          pnpm: `/global/pnpm/bin`,
          npm: `/global/npm/bin`,
          yarn: `/global/yarn/bin`,
          bun: `/global/bun/bin`,
        });

      const mockGetAvailablePackageManagers = jest
        .spyOn(nxpkgUtils, "getAvailablePackageManagers")
        .mockResolvedValue({
          pnpm: packageManager === "pnpm" ? packageManagerVersion : undefined,
          npm: packageManager === "npm" ? packageManagerVersion : undefined,
          yarn: packageManager === "yarn" ? packageManagerVersion : undefined,
          bun: packageManager === "bun" ? packageManagerVersion : undefined,
        });

      const project = getWorkspaceDetailsMockReturnValue({
        root,
        packageManager,
      });
      const mockGetWorkspaceDetails = jest
        .spyOn(nxpkgWorkspaces, "getWorkspaceDetails")
        .mockResolvedValue(project);

      // get the command
      const upgradeCommand = await getNxpkgUpgradeCommand({
        project,
        to: version === "latest" ? undefined : version,
      });

      expect(upgradeCommand).toEqual(expected);

      mockedExec.mockRestore();
      mockGetPackageManagersBinPaths.mockRestore();
      mockGetAvailablePackageManagers.mockRestore();
      mockGetWorkspaceDetails.mockRestore();
    }
  );

  describe("errors", () => {
    test("fails gracefully if no package.json exists", async () => {
      const { root } = useFixture({
        fixture: "no-package",
      });

      const mockedExec = jest
        .spyOn(utils, "exec")
        .mockImplementation((command: string) => {
          // fail the check for the nxpkg to force local
          if (command.includes("bin")) {
            return undefined;
          }
        });

      const mockGetAvailablePackageManagers = jest
        .spyOn(nxpkgUtils, "getAvailablePackageManagers")
        .mockResolvedValue({
          pnpm: "8.0.0",
          npm: undefined,
          yarn: undefined,
          bun: undefined,
        });

      const project = getWorkspaceDetailsMockReturnValue({
        root,
        packageManager: "pnpm",
      });
      const mockGetWorkspaceDetails = jest
        .spyOn(nxpkgWorkspaces, "getWorkspaceDetails")
        .mockResolvedValue(project);

      // get the command
      const upgradeCommand = await getNxpkgUpgradeCommand({
        project,
      });

      expect(upgradeCommand).toEqual(undefined);

      mockedExec.mockRestore();
      mockGetAvailablePackageManagers.mockRestore();
      mockGetWorkspaceDetails.mockRestore();
    });

    test.each([
      {
        fixture: "no-package",
        name: "fails gracefully if no package.json exists",
      },
      {
        fixture: "no-nxpkg",
        name: "fails gracefully if nxpkg cannot be found in package.json",
      },
      {
        fixture: "no-deps",
        name: "fails gracefully if package.json has no deps or devDeps",
      },
    ])("$name", async ({ fixture }) => {
      const { root } = useFixture({
        fixture,
      });

      const mockedExec = jest
        .spyOn(utils, "exec")
        .mockImplementation((command: string) => {
          // fail the check for the nxpkg to force local
          if (command.includes("bin")) {
            return undefined;
          }
        });

      const mockGetAvailablePackageManagers = jest
        .spyOn(nxpkgUtils, "getAvailablePackageManagers")
        .mockResolvedValue({
          pnpm: "8.0.0",
          npm: undefined,
          yarn: undefined,
          bun: undefined,
        });

      const project = getWorkspaceDetailsMockReturnValue({
        root,
        packageManager: "pnpm",
      });
      const mockGetWorkspaceDetails = jest
        .spyOn(nxpkgWorkspaces, "getWorkspaceDetails")
        .mockResolvedValue(project);

      // get the command
      const upgradeCommand = await getNxpkgUpgradeCommand({
        project,
      });

      expect(upgradeCommand).toEqual(undefined);

      mockedExec.mockRestore();
      mockGetAvailablePackageManagers.mockRestore();
      mockGetWorkspaceDetails.mockRestore();
    });
  });
});
