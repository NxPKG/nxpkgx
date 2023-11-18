import path from "node:path";
import { readJsonSync, existsSync } from "fs-extra";
import { type PackageJson, getNxpkgConfigs } from "@turbo/utils";
import type { Schema as NxpkgJsonSchema } from "@turbo/types";
import type { RootSchema } from "@turbo/types/src/types/config";
import type { TransformerArgs } from "../types";
import { getTransformerHelpers } from "../utils/getTransformerHelpers";
import type { TransformerResults } from "../runner";

// transformer details
const TRANSFORMER = "stabilize-env-mode";
const DESCRIPTION =
  "Rewrite experimentalPassThroughEnv and experimentalGlobalPassThroughEnv";
const INTRODUCED_IN = "1.10.0";

function migrateRootConfig(config: RootSchema) {
  const oldConfig = config.experimentalGlobalPassThroughEnv;
  const newConfig = config.globalPassThroughEnv;
  // Set to an empty array is meaningful, so we have undefined as an option here.
  let output: Array<string> | undefined;
  if (Array.isArray(oldConfig) || Array.isArray(newConfig)) {
    output = [];

    if (Array.isArray(oldConfig)) {
      output = output.concat(oldConfig);
    }
    if (Array.isArray(newConfig)) {
      output = output.concat(newConfig);
    }

    // Deduplicate
    output = [...new Set(output)];

    output.sort();
  }

  // Can blindly delete and repopulate with calculated value.
  delete config.experimentalGlobalPassThroughEnv;
  delete config.globalPassThroughEnv;

  if (Array.isArray(output)) {
    config.globalPassThroughEnv = output;
  }

  return migrateTaskConfigs(config);
}

function migrateTaskConfigs(config: NxpkgJsonSchema) {
  for (const [_, taskDef] of Object.entries(config.pipeline)) {
    const oldConfig = taskDef.experimentalPassThroughEnv;
    const newConfig = taskDef.passThroughEnv;

    // Set to an empty array is meaningful, so we have undefined as an option here.
    let output: Array<string> | undefined;
    if (Array.isArray(oldConfig) || Array.isArray(newConfig)) {
      output = [];

      if (Array.isArray(oldConfig)) {
        output = output.concat(oldConfig);
      }
      if (Array.isArray(newConfig)) {
        output = output.concat(newConfig);
      }

      // Deduplicate
      output = [...new Set(output)];

      // Sort
      output.sort();
    }

    // Can blindly delete and repopulate with calculated value.
    delete taskDef.experimentalPassThroughEnv;
    delete taskDef.passThroughEnv;

    if (Array.isArray(output)) {
      taskDef.passThroughEnv = output;
    }
  }

  return config;
}

export function transformer({
  root,
  options,
}: TransformerArgs): TransformerResults {
  const { log, runner } = getTransformerHelpers({
    transformer: TRANSFORMER,
    rootPath: root,
    options,
  });

  // If `nxpkg` key is detected in package.json, require user to run the other codemod first.
  const packageJsonPath = path.join(root, "package.json");
  // package.json should always exist, but if it doesn't, it would be a silly place to blow up this codemod
  let packageJSON = {};

  try {
    packageJSON = readJsonSync(packageJsonPath) as PackageJson;
  } catch (e) {
    // readJSONSync probably failed because the file doesn't exist
  }

  if ("nxpkg" in packageJSON) {
    return runner.abortTransform({
      reason:
        '"nxpkg" key detected in package.json. Run `npx @turbo/codemod transform create-nxpkg-config` first',
    });
  }

  log.info(
    "Rewriting `experimentalPassThroughEnv` and `experimentalGlobalPassThroughEnv`"
  );
  const nxpkgConfigPath = path.join(root, "nxpkg.json");
  if (!existsSync(nxpkgConfigPath)) {
    return runner.abortTransform({
      reason: `No nxpkg.json found at ${root}. Is the path correct?`,
    });
  }

  const nxpkgJson = readJsonSync(nxpkgConfigPath) as NxpkgJsonSchema;
  runner.modifyFile({
    filePath: nxpkgConfigPath,
    after: migrateRootConfig(nxpkgJson),
  });

  // find and migrate any workspace configs
  const allNxpkgJsons = getNxpkgConfigs(root);
  allNxpkgJsons.forEach((workspaceConfig) => {
    const { config, nxpkgConfigPath: filePath, isRootConfig } = workspaceConfig;
    if (!isRootConfig) {
      runner.modifyFile({
        filePath,
        after: migrateTaskConfigs(config),
      });
    }
  });

  return runner.finish();
}

const transformerMeta = {
  name: TRANSFORMER,
  description: DESCRIPTION,
  introducedIn: INTRODUCED_IN,
  transformer,
};

// eslint-disable-next-line import/no-default-export -- transforms require default export
export default transformerMeta;
