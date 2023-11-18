import type { Project } from "@turbo/workspaces";
import type { NxpkgGeneratorCLIOptions } from "../commands/workspace";
import type { CustomGeneratorCLIOptions } from "../commands/run";

export type WorkspaceType = "app" | "package";
export interface CopyData {
  type: "internal" | "external";
  source: string;
}

export type NxpkgGeneratorOptions = Omit<
  NxpkgGeneratorCLIOptions,
  "copy" | "empty"
> & {
  copy: CopyData;
  method: "copy" | "empty";
};

export interface NxpkgGeneratorArguments {
  project: Project;
  opts: NxpkgGeneratorOptions;
}

export interface CustomGeneratorArguments {
  generator: string | undefined;
  project: Project;
  opts: CustomGeneratorCLIOptions;
}
