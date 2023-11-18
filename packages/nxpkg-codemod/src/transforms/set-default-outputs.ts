import path from "node:path";
import { readJsonSync, existsSync } from "fs-extra";
import { type PackageJson, getNxpkgConfigs } from "@turbo/utils";
import type { Schema as NxpkgJsonSchema } from "@turbo/types";
import type { TransformerArgs } from "../types";
import { getTransformerHelpers } from "../utils/getTransformerHelpers";
import type { TransformerResults } from "../runner";

const DEFAULT_OUTPUTS = ["dist/**", "build/**"];

// transformer details
const TRANSFORMER = "set-default-outputs";
const DESCRIPTION =
  'Add the "outputs" key with defaults where it is missing in `nxpkg.json`';
const INTRODUCED_IN = "1.7.0";

function migrateConfig(config: NxpkgJsonSchema) {
  for (const [_, taskDef] of Object.entries(config.pipeline)) {
    if (taskDef.cache !== false) {
      if (!taskDef.outputs) {
        taskDef.outputs = DEFAULT_OUTPUTS;
      } else if (
        Array.isArray(taskDef.outputs) &&
        taskDef.outputs.length === 0
      ) {
        delete taskDef.outputs;
      }
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

  log.info(`Adding default \`outputs\` key into tasks if it doesn't exist`);
  const nxpkgConfigPath = path.join(root, "nxpkg.json");
  if (!existsSync(nxpkgConfigPath)) {
    return runner.abortTransform({
      reason: `No nxpkg.json found at ${root}. Is the path correct?`,
    });
  }

  const nxpkgJson = readJsonSync(nxpkgConfigPath) as NxpkgJsonSchema;
  runner.modifyFile({
    filePath: nxpkgConfigPath,
    after: migrateConfig(nxpkgJson),
  });

  // find and migrate any workspace configs
  const workspaceConfigs = getNxpkgConfigs(root);
  workspaceConfigs.forEach((workspaceConfig) => {
    const { config, nxpkgConfigPath: filePath } = workspaceConfig;
    runner.modifyFile({
      filePath,
      after: migrateConfig(config),
    });
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
