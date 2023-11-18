import path from "node:path";
import { existsSync } from "fs-extra";

const HINTS = ["package.json", "nxpkg.json", ".git"];

export function looksLikeRepo({ directory }: { directory: string }): boolean {
  return HINTS.some((hint) => existsSync(path.join(directory, hint)));
}
