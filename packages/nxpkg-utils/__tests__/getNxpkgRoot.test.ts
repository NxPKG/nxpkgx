import path from "path";
import { getNxpkgRoot } from "../src/getNxpkgRoot";
import { setupTestFixtures } from "@turbo/test-utils";

describe("getNxpkgConfigs", () => {
  const { useFixture } = setupTestFixtures({
    directory: path.join(__dirname, "../"),
    test: "common",
  });

  test.each([[""], ["child"]])(
    "finds the root in a non-monorepo (%s)",
    (repoPath) => {
      const { root } = useFixture({ fixture: `single-package` });
      const nxpkgRoot = getNxpkgRoot(path.join(root, repoPath));
      expect(nxpkgRoot).toEqual(root);
    }
  );

  test.each([
    [""],
    ["apps"],
    ["apps/docs"],
    ["apps/web"],
    ["packages"],
    ["packages/ui"],
    ["not-a-real/path"],
  ])("finds the root in a monorepo with workspace configs (%s)", (repoPath) => {
    const { root } = useFixture({ fixture: `workspace-configs` });
    const nxpkgRoot = getNxpkgRoot(path.join(root, repoPath));
    expect(nxpkgRoot).toEqual(root);
  });
});
