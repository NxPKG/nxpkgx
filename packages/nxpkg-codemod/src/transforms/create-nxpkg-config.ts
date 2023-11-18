import path from "node:path";
import { readJsonSync, existsSync } from "fs-extra";
import { type PackageJson } from "@turbo/utils";
import type { Schema } from "@turbo/types";
import type { TransformerResults } from "../runner";
import { getTransformerHelpers } from "../utils/getTransformerHelpers";
import type { TransformerArgs } from "../types";

// transformer details
const TRANSFORMER = "create-nxpkg-config";
const DESCRIPTION =
  'Create the `nxpkg.json` file from an existing "nxpkg" key in `package.json`';
const INTRODUCED_IN = "1.1.0";

export function transformer({
  root,
  options,
}: TransformerArgs): TransformerResults {
  const { log, runner } = getTransformerHelpers({
    transformer: TRANSFORMER,
    rootPath: root,
    options,
  });

  log.info(`Migrating "package.json" "nxpkg" key to "nxpkg.json" file...`);
  const nxpkgConfigPath = path.join(root, "nxpkg.json");
  const rootPackageJsonPath = path.join(root, "package.json");
  if (!existsSync(rootPackageJsonPath)) {
    return runner.abortTransform({
      reason: `No package.json found at ${root}. Is the path correct?`,
    });
  }

  // read files
  const rootPackageJson = readJsonSync(rootPackageJsonPath) as PackageJson;
  let rootNxpkgJson = null;
  try {
    rootNxpkgJson = readJsonSync(nxpkgConfigPath) as Schema;
  } catch (err) {
    rootNxpkgJson = null;
  }

  // modify files
  let transformedPackageJson = rootPackageJson;
  let transformedNxpkgConfig = rootNxpkgJson;
  if (!rootNxpkgJson && rootPackageJson.nxpkg) {
    const { nxpkg: nxpkgConfig, ...remainingPkgJson } = rootPackageJson;
    transformedNxpkgConfig = nxpkgConfig;
    transformedPackageJson = remainingPkgJson;
  }

  runner.modifyFile({
    filePath: nxpkgConfigPath,
    after: transformedNxpkgConfig,
  });
  runner.modifyFile({
    filePath: rootPackageJsonPath,
    after: transformedPackageJson,
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
