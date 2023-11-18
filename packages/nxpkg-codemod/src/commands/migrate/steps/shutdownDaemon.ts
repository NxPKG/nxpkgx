import type { ExecSyncOptions } from "node:child_process";
import type { Project } from "@turbo/workspaces";
import { exec } from "../utils";

export function shutdownDaemon({ project }: { project: Project }) {
  try {
    const execOpts: ExecSyncOptions = {
      cwd: project.paths.root,
      stdio: "ignore",
    };
    // see if we have a global install
    const nxpkgBinaryPathFromGlobal = exec(`nxpkg bin`, execOpts);
    // if we do, shut it down
    if (nxpkgBinaryPathFromGlobal) {
      exec(`nxpkg daemon stop`, execOpts);
    } else {
      // call nxpkg using the project package manager to shut down the daemon
      let command = `${project.packageManager} nxpkg daemon stop`;
      if (project.packageManager === "npm") {
        command = `npm exec -c 'nxpkg daemon stop'`;
      }

      exec(command, execOpts);
    }
  } catch (e) {
    // skip
  }
}
