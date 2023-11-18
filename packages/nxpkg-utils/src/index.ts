// utils
export { getNxpkgRoot } from "./getNxpkgRoot";
export { getNxpkgConfigs, getWorkspaceConfigs } from "./getNxpkgConfigs";
export { searchUp } from "./searchUp";
export {
  getAvailablePackageManagers,
  getPackageManagersBinPaths,
} from "./managers";
export { isFolderEmpty } from "./isFolderEmpty";
export { validateDirectory } from "./validateDirectory";
export {
  isUrlOk,
  getRepoInfo,
  hasRepo,
  existsInRepo,
  downloadAndExtractRepo,
  downloadAndExtractExample,
} from "./examples";
export { isWriteable } from "./isWriteable";
export { createProject, DownloadError } from "./createProject";
export { convertCase } from "./convertCase";

export * as logger from "./logger";

// types
export type { RepoInfo } from "./examples";
export type {
  NxpkgConfig,
  NxpkgConfigs,
  WorkspaceConfig,
} from "./getNxpkgConfigs";
export * from "./types";
