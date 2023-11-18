import { type Project } from "@turbo/workspaces";
import { exec } from "../utils";
import type { MigrateCommandOptions } from "../types";

export function getCurrentVersion(
  project: Project,
  opts: MigrateCommandOptions
): string | undefined {
  const { from } = opts;
  if (from) {
    return from;
  }

  // try global first
  const nxpkgVersionFromGlobal = exec(`nxpkg --version`, {
    cwd: project.paths.root,
  });

  if (nxpkgVersionFromGlobal) {
    return nxpkgVersionFromGlobal;
  }

  const { packageManager } = project;
  if (packageManager === "yarn") {
    return exec(`yarn nxpkg --version`, { cwd: project.paths.root });
  }
  if (packageManager === "pnpm") {
    return exec(`pnpm nxpkg --version`, { cwd: project.paths.root });
  }

  return exec(`npm exec -c 'nxpkg --version'`, { cwd: project.paths.root });
}
