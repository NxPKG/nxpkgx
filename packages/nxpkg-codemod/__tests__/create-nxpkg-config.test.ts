import { setupTestFixtures } from "@turbo/test-utils";
import fs from "fs-extra";
import { transformer } from "../src/transforms/create-nxpkg-config";

describe("create-nxpkg-config", () => {
  const { useFixture } = setupTestFixtures({
    directory: __dirname,
    test: "create-nxpkg-config",
  });

  test("package.json config exists but no nxpkg.json config - basic", () => {
    // load the fixture for the test
    const { root, read } = useFixture({ fixture: "no-nxpkg-json-config" });

    // nxpkg.json should not exist
    expect(read("nxpkg.json")).toBeUndefined();

    // get config from package.json for comparison later
    const nxpkgConfig = JSON.parse(read("package.json") || "{}").nxpkg;
    expect(nxpkgConfig).toBeDefined();
    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    // nxpkg.json should now exist (and match the package.json config)
    expect(JSON.parse(read("nxpkg.json") || "{}")).toEqual(nxpkgConfig);

    // result should be correct
    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "package.json": Object {
          "action": "modified",
          "additions": 0,
          "deletions": 1,
        },
        "nxpkg.json": Object {
          "action": "modified",
          "additions": 1,
          "deletions": 0,
        },
      }
    `);
  });

  test("package.json config exists but no nxpkg.json config - repeat run", () => {
    // load the fixture for the test
    const { root, read } = useFixture({ fixture: "no-nxpkg-json-config" });

    // nxpkg.json should not exist
    expect(read("nxpkg.json")).toBeUndefined();

    // get config from package.json for comparison later
    const nxpkgConfig = JSON.parse(read("package.json") || "{}").nxpkg;
    expect(nxpkgConfig).toBeDefined();
    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    // nxpkg.json should now exist (and match the package.json config)
    expect(JSON.parse(read("nxpkg.json") || "{}")).toEqual(nxpkgConfig);

    // result should be correct
    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "package.json": Object {
          "action": "modified",
          "additions": 0,
          "deletions": 1,
        },
        "nxpkg.json": Object {
          "action": "modified",
          "additions": 1,
          "deletions": 0,
        },
      }
    `);

    // run the transformer
    const repeatResult = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });
    // result should be correct
    expect(repeatResult.fatalError).toBeUndefined();
    expect(repeatResult.changes).toMatchInlineSnapshot(`
      Object {
        "package.json": Object {
          "action": "unchanged",
          "additions": 0,
          "deletions": 0,
        },
        "nxpkg.json": Object {
          "action": "unchanged",
          "additions": 0,
          "deletions": 0,
        },
      }
    `);
  });

  test("package.json config exists but no nxpkg.json config - dry", () => {
    // load the fixture for the test
    const { root, read } = useFixture({ fixture: "no-nxpkg-json-config" });

    // nxpkg.json should not exist
    expect(read("nxpkg.json")).toBeUndefined();

    // get config from package.json for comparison later
    const nxpkgConfig = JSON.parse(read("package.json") || "{}").nxpkg;
    expect(nxpkgConfig).toBeDefined();
    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: true, print: false },
    });

    // nxpkg.json still not exist (dry run)
    expect(read("nxpkg.json")).toBeUndefined();

    // result should be correct
    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "package.json": Object {
          "action": "skipped",
          "additions": 0,
          "deletions": 1,
        },
        "nxpkg.json": Object {
          "action": "skipped",
          "additions": 1,
          "deletions": 0,
        },
      }
    `);
  });

  test("package.json config exists but no nxpkg.json config - print", () => {
    // load the fixture for the test
    const { root, read } = useFixture({ fixture: "no-nxpkg-json-config" });

    // nxpkg.json should not exist
    expect(read("nxpkg.json")).toBeUndefined();

    // get config from package.json for comparison later
    const nxpkgConfig = JSON.parse(read("package.json") || "{}").nxpkg;
    expect(nxpkgConfig).toBeDefined();
    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: true },
    });

    // nxpkg.json should now exist (and match the package.json config)
    expect(JSON.parse(read("nxpkg.json") || "{}")).toEqual(nxpkgConfig);

    // result should be correct
    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "package.json": Object {
          "action": "modified",
          "additions": 0,
          "deletions": 1,
        },
        "nxpkg.json": Object {
          "action": "modified",
          "additions": 1,
          "deletions": 0,
        },
      }
    `);
  });

  test("package.json config exists but no nxpkg.json config - dry & print", () => {
    // load the fixture for the test
    const { root, read } = useFixture({ fixture: "no-nxpkg-json-config" });

    // nxpkg.json should not exist
    expect(read("nxpkg.json")).toBeUndefined();

    // get config from package.json for comparison later
    const nxpkgConfig = JSON.parse(read("package.json") || "{}").nxpkg;
    expect(nxpkgConfig).toBeDefined();
    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: true, print: true },
    });

    // nxpkg.json still not exist (dry run)
    expect(read("nxpkg.json")).toBeUndefined();

    // result should be correct
    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "package.json": Object {
          "action": "skipped",
          "additions": 0,
          "deletions": 1,
        },
        "nxpkg.json": Object {
          "action": "skipped",
          "additions": 1,
          "deletions": 0,
        },
      }
    `);
  });

  test("no package.json config or nxpkg.json file exists", () => {
    // load the fixture for the test
    const { root, read } = useFixture({ fixture: "no-package-json-config" });

    // nxpkg.json should not exist
    expect(read("nxpkg.json")).toBeUndefined();

    // get config from package.json for comparison later
    const packageJsonConfig = JSON.parse(read("package.json") || "{}");
    const nxpkgConfig = packageJsonConfig.nxpkg;
    expect(nxpkgConfig).toBeUndefined();
    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    // nxpkg.json should still not exist
    expect(read("nxpkg.json")).toBeUndefined();

    // make sure we didn't change the package.json
    expect(JSON.parse(read("package.json") || "{}")).toEqual(packageJsonConfig);

    // result should be correct
    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "package.json": Object {
          "action": "unchanged",
          "additions": 0,
          "deletions": 0,
        },
        "nxpkg.json": Object {
          "action": "unchanged",
          "additions": 0,
          "deletions": 0,
        },
      }
    `);
  });

  test("no package.json file exists", () => {
    // load the fixture for the test
    const { root, read } = useFixture({ fixture: "no-package-json-file" });

    // nxpkg.json should not exist
    expect(read("nxpkg.json")).toBeUndefined();

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    // nxpkg.json should still not exist
    expect(read("nxpkg.json")).toBeUndefined();

    // result should be correct
    expect(result.fatalError?.message).toMatch(
      /No package\.json found at .*?\. Is the path correct\?/
    );
  });

  test("nxpkg.json file exists and no package.json config exists", () => {
    // load the fixture for the test
    const { root, read } = useFixture({ fixture: "nxpkg-json-config" });

    // nxpkg.json should exist
    expect(read("nxpkg.json")).toBeDefined();

    // no config should exist in package.json
    const packageJsonConfig = JSON.parse(read("package.json") || "{}");
    const nxpkgConfig = packageJsonConfig.nxpkg;
    expect(nxpkgConfig).toBeUndefined();

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    // nxpkg.json should still exist
    expect(read("nxpkg.json")).toBeDefined();

    // make sure we didn't change the package.json
    expect(JSON.parse(read("package.json") || "{}")).toEqual(packageJsonConfig);

    // result should be correct
    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "package.json": Object {
          "action": "unchanged",
          "additions": 0,
          "deletions": 0,
        },
        "nxpkg.json": Object {
          "action": "unchanged",
          "additions": 0,
          "deletions": 0,
        },
      }
    `);
  });

  test("nxpkg.json file exists and package.json config exists", () => {
    // load the fixture for the test
    const { root, read } = useFixture({ fixture: "both-configs" });

    // nxpkg.json should exist
    const nxpkgJsonConfig = JSON.parse(read("nxpkg.json") || "{}");
    expect(nxpkgJsonConfig.pipeline).toBeDefined();

    // no config should exist in package.json
    const packageJsonConfig = JSON.parse(read("package.json") || "{}");
    const nxpkgConfig = JSON.parse(read("package.json") || "{}").nxpkg;
    expect(nxpkgConfig).toBeDefined();

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    // make sure we didn't change the package.json
    expect(JSON.parse(read("package.json") || "{}")).toEqual(packageJsonConfig);

    // make sure we didn't change the nxpkg.json
    expect(JSON.parse(read("nxpkg.json") || "{}")).toEqual(nxpkgJsonConfig);

    // result should be correct
    expect(result.fatalError?.message).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "package.json": Object {
          "action": "unchanged",
          "additions": 0,
          "deletions": 0,
        },
        "nxpkg.json": Object {
          "action": "unchanged",
          "additions": 0,
          "deletions": 0,
        },
      }
    `);
  });

  test("errors when unable to write json", () => {
    // load the fixture for the test
    const { root, read } = useFixture({ fixture: "no-nxpkg-json-config" });

    // nxpkg.json should not exist
    expect(read("nxpkg.json")).toBeUndefined();

    // get config from package.json for comparison later
    const nxpkgConfig = JSON.parse(read("package.json") || "{}").nxpkg;
    expect(nxpkgConfig).toBeDefined();

    const mockWriteJsonSync = jest
      .spyOn(fs, "writeJsonSync")
      .mockImplementation(() => {
        throw new Error("could not write file");
      });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    // nxpkg.json should still not exist (error writing)
    expect(read("nxpkg.json")).toBeUndefined();

    // result should be correct
    expect(result.fatalError).toBeDefined();
    expect(result.fatalError?.message).toMatch(
      "Encountered an error while transforming files"
    );
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "package.json": Object {
          "action": "error",
          "additions": 0,
          "deletions": 1,
          "error": [Error: could not write file],
        },
        "nxpkg.json": Object {
          "action": "error",
          "additions": 1,
          "deletions": 0,
          "error": [Error: could not write file],
        },
      }
    `);

    mockWriteJsonSync.mockRestore();
  });
});
