import fs from "node:fs";
import path from "node:path";
import yaml from "js-yaml";
import { sync } from "fast-glob";
import type { Schema } from "@turbo/types";
import JSON5 from "json5";
import * as logger from "./logger";
import { getNxpkgRoot } from "./getNxpkgRoot";
import type { PackageJson, PNPMWorkspaceConfig } from "./types";

const ROOT_GLOB = "nxpkg.json";
const ROOT_WORKSPACE_GLOB = "package.json";

export interface WorkspaceConfig {
  workspaceName: string;
  workspacePath: string;
  isWorkspaceRoot: boolean;
  nxpkgConfig?: Schema;
}

export interface NxpkgConfig {
  config: Schema;
  nxpkgConfigPath: string;
  workspacePath: string;
  isRootConfig: boolean;
}

export type NxpkgConfigs = Array<NxpkgConfig>;

interface Options {
  cache?: boolean;
}

const nxpkgConfigsCache: Record<string, NxpkgConfigs> = {};
const workspaceConfigCache: Record<string, Array<WorkspaceConfig>> = {};

// A quick and dirty workspace parser
// TODO: after @turbo/workspace-convert is merged, we can leverage those utils here
function getWorkspaceGlobs(root: string): Array<string> {
  try {
    if (fs.existsSync(path.join(root, "pnpm-workspace.yaml"))) {
      const workspaceConfig = yaml.load(
        fs.readFileSync(path.join(root, "pnpm-workspace.yaml"), "utf8")
      ) as PNPMWorkspaceConfig;

      return workspaceConfig.packages || [];
    }
    const packageJson = JSON.parse(
      fs.readFileSync(path.join(root, "package.json"), "utf8")
    ) as PackageJson;
    if (packageJson.workspaces) {
      // support nested packages workspace format
      if ("packages" in packageJson.workspaces) {
        return packageJson.workspaces.packages || [];
      }

      if (Array.isArray(packageJson.workspaces)) {
        return packageJson.workspaces;
      }
    }
    return [];
  } catch (e) {
    return [];
  }
}

export function getNxpkgConfigs(cwd?: string, opts?: Options): NxpkgConfigs {
  const nxpkgRoot = getNxpkgRoot(cwd, opts);
  const configs: NxpkgConfigs = [];

  const cacheEnabled = opts?.cache ?? true;
  if (cacheEnabled && cwd && cwd in nxpkgConfigsCache) {
    return nxpkgConfigsCache[cwd];
  }

  // parse workspaces
  if (nxpkgRoot) {
    const workspaceGlobs = getWorkspaceGlobs(nxpkgRoot);
    const workspaceConfigGlobs = workspaceGlobs.map(
      (glob) => `${glob}/nxpkg.json`
    );

    const configPaths = sync([ROOT_GLOB, ...workspaceConfigGlobs], {
      cwd: nxpkgRoot,
      onlyFiles: true,
      followSymbolicLinks: false,
      // avoid throwing when encountering permission errors or unreadable paths
      suppressErrors: true,
    }).map((configPath) => path.join(nxpkgRoot, configPath));

    configPaths.forEach((configPath) => {
      try {
        const raw = fs.readFileSync(configPath, "utf8");
        // eslint-disable-next-line import/no-named-as-default-member -- json5 exports different objects depending on if you're using esm or cjs (https://github.com/json5/json5/issues/240)
        const nxpkgJsonContent: Schema = JSON5.parse(raw);
        // basic config validation
        const isRootConfig = path.dirname(configPath) === nxpkgRoot;
        if (isRootConfig) {
          // invalid - root config with extends
          if ("extends" in nxpkgJsonContent) {
            return;
          }
        } else if (!("extends" in nxpkgJsonContent)) {
          // invalid - workspace config with no extends
          return;
        }
        configs.push({
          config: nxpkgJsonContent,
          nxpkgConfigPath: configPath,
          workspacePath: path.dirname(configPath),
          isRootConfig,
        });
      } catch (e) {
        // if we can't read or parse the config, just ignore it with a warning
        logger.warn(e);
      }
    });
  }

  if (cacheEnabled && cwd) {
    nxpkgConfigsCache[cwd] = configs;
  }

  return configs;
}

export function getWorkspaceConfigs(
  cwd?: string,
  opts?: Options
): Array<WorkspaceConfig> {
  const nxpkgRoot = getNxpkgRoot(cwd, opts);
  const configs: Array<WorkspaceConfig> = [];

  const cacheEnabled = opts?.cache ?? true;
  if (cacheEnabled && cwd && cwd in workspaceConfigCache) {
    return workspaceConfigCache[cwd];
  }

  // parse workspaces
  if (nxpkgRoot) {
    const workspaceGlobs = getWorkspaceGlobs(nxpkgRoot);
    const workspaceConfigGlobs = workspaceGlobs.map(
      (glob) => `${glob}/package.json`
    );

    const configPaths = sync([ROOT_WORKSPACE_GLOB, ...workspaceConfigGlobs], {
      cwd: nxpkgRoot,
      onlyFiles: true,
      followSymbolicLinks: false,
      // avoid throwing when encountering permission errors or unreadable paths
      suppressErrors: true,
    }).map((configPath) => path.join(nxpkgRoot, configPath));

    configPaths.forEach((configPath) => {
      try {
        const rawPackageJson = fs.readFileSync(configPath, "utf8");
        const packageJsonContent = JSON.parse(rawPackageJson) as PackageJson;

        const workspaceName = packageJsonContent.name;
        const workspacePath = path.dirname(configPath);
        const isWorkspaceRoot = workspacePath === nxpkgRoot;

        // Try and get nxpkg.json
        const nxpkgJsonPath = path.join(workspacePath, "nxpkg.json");
        let rawNxpkgJson = null;
        let nxpkgConfig: Schema | undefined;
        try {
          rawNxpkgJson = fs.readFileSync(nxpkgJsonPath, "utf8");
          // eslint-disable-next-line import/no-named-as-default-member -- json5 exports different objects depending on if you're using esm or cjs (https://github.com/json5/json5/issues/240)
          nxpkgConfig = JSON5.parse(rawNxpkgJson);

          if (nxpkgConfig) {
            // basic config validation
            if (isWorkspaceRoot) {
              // invalid - root config with extends
              if ("extends" in nxpkgConfig) {
                return;
              }
            } else if (!("extends" in nxpkgConfig)) {
              // invalid - workspace config with no extends
              return;
            }
          }
        } catch (e) {
          // It is fine for there to not be a nxpkg.json.
        }

        configs.push({
          workspaceName,
          workspacePath,
          isWorkspaceRoot,
          nxpkgConfig,
        });
      } catch (e) {
        // if we can't read or parse the config, just ignore it with a warning
        logger.warn(e);
      }
    });
  }

  if (cacheEnabled && cwd) {
    workspaceConfigCache[cwd] = configs;
  }

  return configs;
}
