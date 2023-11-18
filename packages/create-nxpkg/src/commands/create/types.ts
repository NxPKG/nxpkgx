import type { PackageManager } from "@turbo/utils";

export type CreateCommandArgument = string | undefined;

export interface CreateCommandOptions {
  packageManager?: PackageManager;
  skipInstall?: boolean;
  skipTransforms?: boolean;
  nxpkgVersion?: string;
  example?: string;
  examplePath?: string;
}
