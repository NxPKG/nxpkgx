import { setupTestFixtures } from "@turbo/test-utils";
import { type Schema } from "@turbo/types";
import { transformer } from "../src/transforms/set-default-outputs";

describe("set-default-outputs", () => {
  const { useFixture } = setupTestFixtures({
    directory: __dirname,
    test: "set-default-outputs",
  });
  it("migrates nxpkg.json outputs - basic", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "old-outputs",
    });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      pipeline: {
        "build-one": {
          outputs: ["foo"],
        },
        "build-two": {},
        "build-three": {
          outputs: ["dist/**", "build/**"],
        },
      },
    });

    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "nxpkg.json": Object {
          "action": "modified",
          "additions": 2,
          "deletions": 1,
        },
      }
    `);
  });

  it("migrates nxpkg.json outputs - workspace configs", () => {
    // load the fixture for the test
    const { root, readJson } = useFixture({
      fixture: "workspace-configs",
    });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    expect(readJson("nxpkg.json") || "{}").toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      pipeline: {
        "build-one": {
          outputs: ["foo"],
        },
        "build-two": {},
        "build-three": {
          outputs: ["dist/**", "build/**"],
        },
      },
    });

    expect(readJson("apps/docs/nxpkg.json") || "{}").toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      extends: ["//"],
      pipeline: {
        build: {
          outputs: ["dist/**", "build/**"],
        },
      },
    });

    expect(readJson("apps/web/nxpkg.json") || "{}").toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      extends: ["//"],
      pipeline: {
        build: {},
      },
    });

    expect(readJson("packages/ui/nxpkg.json") || "{}").toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      extends: ["//"],
      pipeline: {
        "build-three": {
          outputs: ["dist/**", "build/**"],
        },
      },
    });

    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "apps/docs/nxpkg.json": Object {
          "action": "modified",
          "additions": 1,
          "deletions": 1,
        },
        "apps/web/nxpkg.json": Object {
          "action": "modified",
          "additions": 1,
          "deletions": 0,
        },
        "packages/ui/nxpkg.json": Object {
          "action": "modified",
          "additions": 1,
          "deletions": 1,
        },
        "nxpkg.json": Object {
          "action": "modified",
          "additions": 2,
          "deletions": 1,
        },
      }
    `);
  });

  it("migrates nxpkg.json outputs - dry", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "old-outputs",
    });

    const nxpkgJson = JSON.parse(read("nxpkg.json") || "{}") as Schema;

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: true, print: false },
    });

    // make sure it didn't change
    expect(JSON.parse(read("nxpkg.json") || "{}")).toEqual(nxpkgJson);

    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "nxpkg.json": Object {
          "action": "skipped",
          "additions": 2,
          "deletions": 1,
        },
      }
    `);
  });

  it("migrates nxpkg.json outputs - print", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "old-outputs",
    });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: true },
    });

    expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      pipeline: {
        "build-one": {
          outputs: ["foo"],
        },
        "build-two": {},
        "build-three": {
          outputs: ["dist/**", "build/**"],
        },
      },
    });

    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "nxpkg.json": Object {
          "action": "modified",
          "additions": 2,
          "deletions": 1,
        },
      }
    `);
  });

  it("migrates nxpkg.json outputs - dry & print", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "old-outputs",
    });

    const nxpkgJson = JSON.parse(read("nxpkg.json") || "{}") as Schema;

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: true, print: false },
    });

    // make sure it didn't change
    expect(JSON.parse(read("nxpkg.json") || "{}")).toEqual(nxpkgJson);

    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "nxpkg.json": Object {
          "action": "skipped",
          "additions": 2,
          "deletions": 1,
        },
      }
    `);
  });

  it("migrates nxpkg.json outputs - invalid", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "invalid-outputs",
    });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      pipeline: {
        "build-one": {
          outputs: ["foo"],
        },
        "build-two": {},
        "build-three": {
          outputs: ["dist/**", "build/**"],
        },
        "garbage-in-numeric-0": {
          outputs: ["dist/**", "build/**"],
        },
        "garbage-in-numeric": {
          outputs: 42,
        },
        "garbage-in-string": {
          outputs: "string",
        },
        "garbage-in-empty-string": {
          outputs: ["dist/**", "build/**"],
        },
        "garbage-in-null": {
          outputs: ["dist/**", "build/**"],
        },
        "garbage-in-false": {
          outputs: ["dist/**", "build/**"],
        },
        "garbage-in-true": {
          outputs: true,
        },
        "garbage-in-object": {
          outputs: {},
        },
      },
    });

    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "nxpkg.json": Object {
          "action": "modified",
          "additions": 6,
          "deletions": 5,
        },
      }
    `);
  });

  it("migrates nxpkg.json outputs - config with no pipeline", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "no-pipeline",
    });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      globalDependencies: ["$NEXT_PUBLIC_API_KEY", "$STRIPE_API_KEY", ".env"],
      pipeline: {},
    });

    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "nxpkg.json": Object {
          "action": "unchanged",
          "additions": 0,
          "deletions": 0,
        },
      }
    `);
  });

  it("migrates nxpkg.json outputs - config with no outputs", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "no-outputs",
    });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      pipeline: {
        "build-one": {
          dependsOn: ["build-two"],
          outputs: ["dist/**", "build/**"],
        },
        "build-two": {
          cache: false,
        },
        "build-three": {
          persistent: true,
          outputs: ["dist/**", "build/**"],
        },
      },
    });

    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "nxpkg.json": Object {
          "action": "modified",
          "additions": 2,
          "deletions": 0,
        },
      }
    `);
  });

  it("errors if no nxpkg.json can be found", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "no-nxpkg-json",
    });

    expect(read("nxpkg.json")).toBeUndefined();

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    expect(read("nxpkg.json")).toBeUndefined();
    expect(result.fatalError).toBeDefined();
    expect(result.fatalError?.message).toMatch(
      /No nxpkg\.json found at .*?\. Is the path correct\?/
    );
  });

  it("errors if package.json config exists and has not been migrated", () => {
    // load the fixture for the test
    const { root } = useFixture({
      fixture: "old-config",
    });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    expect(result.fatalError).toBeDefined();
    expect(result.fatalError?.message).toMatch(
      'nxpkg" key detected in package.json. Run `npx @turbo/codemod transform create-nxpkg-config` first'
    );
  });
});
