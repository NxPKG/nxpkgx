import { setupTestFixtures } from "@turbo/test-utils";
import { transformer } from "../src/transforms/transform-env-literals-to-wildcards";

describe.only("transform-env-literals-to-wildcards", () => {
  const { useFixture } = setupTestFixtures({
    directory: __dirname,
    test: "transform-env-literals-to-wildcards",
  });

  it("migrates wildcards has-empty", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "has-empty",
    });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      globalEnv: [],
      globalPassThroughEnv: [],
      pipeline: {
        build: {
          env: [],
          passThroughEnv: [],
        },
      },
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

  it("migrates env-mode has-nothing", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "has-nothing",
    });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      pipeline: {
        build: {},
      },
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

  it("migrates env-mode needs-rewriting", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "needs-rewriting",
    });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      globalEnv: ["NO!", "\\!!!", "\\!!!"],
      globalPassThroughEnv: ["DOES", "\\*\\*BOLD\\*\\*", "WORK"],
      pipeline: {
        build: {
          env: ["PLAIN", "SMALL_PRINT\\*"],
          passThroughEnv: ["PASSWORD", "\\*\\*\\*\\*\\*"],
        },
      },
    });

    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "nxpkg.json": Object {
          "action": "modified",
          "additions": 4,
          "deletions": 4,
        },
      }
    `);
  });

  it("migrates env-mode workspace-configs", () => {
    // load the fixture for the test
    const { root, read } = useFixture({
      fixture: "workspace-configs",
    });

    // run the transformer
    const result = transformer({
      root,
      options: { force: false, dry: false, print: false },
    });

    expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
      $schema: "https://nxpkg.build/schema.json",
      globalEnv: ["\\!\\*!\\*"],
      globalPassThroughEnv: ["\\!\\*!\\*"],
      pipeline: {
        build: {
          env: ["NO_ROOT_ENV", "\\!\\*!\\*ROOT"],
          passThroughEnv: ["NO_ROOT_PASSTHROUGH_ENV", "\\!\\*!\\*ROOT"],
        },
      },
    });

    expect(JSON.parse(read("apps/docs/nxpkg.json") || "{}")).toStrictEqual({
      extends: ["//"],
      pipeline: {
        build: {
          env: ["NO_DOCS_ENV", "\\!\\*!\\*DOCS"],
          passThroughEnv: ["NO_DOCS_PASSTHROUGH_ENV", "\\!\\*!\\*DOCS"],
        },
      },
    });

    expect(JSON.parse(read("apps/website/nxpkg.json") || "{}")).toStrictEqual({
      extends: ["//"],
      pipeline: {
        build: {
          env: ["NO_WEBSITE_ENV", "\\!\\*!\\*WEBSITE"],
          passThroughEnv: ["NO_WEBSITE_PASSTHROUGH_ENV", "\\!\\*!\\*WEBSITE"],
        },
      },
    });

    expect(result.fatalError).toBeUndefined();
    expect(result.changes).toMatchInlineSnapshot(`
      Object {
        "apps/docs/nxpkg.json": Object {
          "action": "modified",
          "additions": 2,
          "deletions": 2,
        },
        "apps/website/nxpkg.json": Object {
          "action": "modified",
          "additions": 2,
          "deletions": 2,
        },
        "nxpkg.json": Object {
          "action": "modified",
          "additions": 4,
          "deletions": 4,
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
