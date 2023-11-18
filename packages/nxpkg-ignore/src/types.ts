export type NonFatalErrorKey =
  | "MISSING_LOCKFILE"
  | "NO_PACKAGE_MANAGER"
  | "UNREACHABLE_PARENT"
  | "INVALID_COMPARISON";

export interface NonFatalError {
  regex: Array<RegExp>;
  message: string;
}

export type NonFatalErrors = Record<NonFatalErrorKey, NonFatalError>;

export type NxpkgIgnoreArg = string | undefined;

export interface NxpkgIgnoreOptions {
  // the working directory to use when looking for a workspace
  directory?: string;
  // the workspace to check for changes
  workspace?: string;
  // the task to run, if not build
  task?: string;
  // A ref/head to compare against if no previously deployed SHA is available
  fallback?: string;
  // The maxBuffer for the child process in KB
  maxBuffer?: number;
}
