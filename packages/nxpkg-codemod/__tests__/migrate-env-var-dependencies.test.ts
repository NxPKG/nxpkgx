import merge from "deepmerge";
import type { Schema } from "@turbo/types";
import { setupTestFixtures } from "@turbo/test-utils";
import {
  hasLegacyEnvVarDependencies,
  migratePipeline,
  migrateConfig,
  transformer,
} from "../src/transforms/migrate-env-var-dependencies";

const getTestNxpkgConfig = (override: Schema = { pipeline: {} }): Schema => {
  const config = {
    $schema: "./docs/public/schema.json",
    globalDependencies: ["$GLOBAL_ENV_KEY"],
    pipeline: {
      test: {
        outputs: ["coverage/**/*"],
        dependsOn: ["^build"],
      },
      lint: {
        outputs: [],
      },
      dev: {
        cache: false,
      },
      build: {
        outputs: ["dist/**/*", ".next/**/*", "!.next/cache/**"],
        dependsOn: ["^build", "$TASK_ENV_KEY", "$ANOTHER_ENV_KEY"],
      },
    },
  };

  return merge(config, override, {
    arrayMerge: (_, sourceArray) => sourceArray,
  });
};

describe("migrate-env-var-dependencies", () => {
  describe("hasLegacyEnvVarDependencies - utility", () => {
    it("finds env keys in legacy nxpkg.json - has keys", () => {
      const config = getTestNxpkgConfig();
      const { hasKeys, envVars } = hasLegacyEnvVarDependencies(config);
      expect(hasKeys).toEqual(true);
      expect(envVars).toMatchInlineSnapshot(`
              Array [
                "$GLOBAL_ENV_KEY",
                "$TASK_ENV_KEY",
                "$ANOTHER_ENV_KEY",
              ]
          `);
    });

    it("finds env keys in legacy nxpkg.json - multiple pipeline keys", () => {
      const config = getTestNxpkgConfig({
        pipeline: { test: { dependsOn: ["$MY_ENV"] } },
      });
      const { hasKeys, envVars } = hasLegacyEnvVarDependencies(config);
      expect(hasKeys).toEqual(true);
      expect(envVars).toMatchInlineSnapshot(`
              Array [
                "$GLOBAL_ENV_KEY",
                "$MY_ENV",
                "$TASK_ENV_KEY",
                "$ANOTHER_ENV_KEY",
              ]
          `);
    });

    it("finds env keys in legacy nxpkg.json - no keys", () => {
      // override to exclude keys
      const config = getTestNxpkgConfig({
        globalDependencies: [],
        pipeline: { build: { dependsOn: [] } },
      });
      const { hasKeys, envVars } = hasLegacyEnvVarDependencies(config);
      expect(hasKeys).toEqual(false);
      expect(envVars).toMatchInlineSnapshot(`Array []`);
    });

    it("finds env keys in nxpkg.json - no global", () => {
      const { hasKeys, envVars } = hasLegacyEnvVarDependencies({
        pipeline: { build: { dependsOn: ["$cool"] } },
      });
      expect(hasKeys).toEqual(true);
      expect(envVars).toMatchInlineSnapshot(`
        Array [
          "$cool",
        ]
      `);
    });
  });

  describe("migratePipeline - utility", () => {
    it("migrates pipeline with env var dependencies", () => {
      const config = getTestNxpkgConfig();
      const { build } = config.pipeline;
      const pipeline = migratePipeline(build);
      expect(pipeline).toHaveProperty("env");
      expect(pipeline.env).toMatchInlineSnapshot(`
        Array [
          "TASK_ENV_KEY",
          "ANOTHER_ENV_KEY",
        ]
      `);
      expect(pipeline.dependsOn).toMatchInlineSnapshot(`
        Array [
          "^build",
        ]
      `);
    });

    it("migrates pipeline with no env var dependencies", () => {
      const config = getTestNxpkgConfig();
      const { test } = config.pipeline;
      const pipeline = migratePipeline(test);
      expect(pipeline.env).toBeUndefined();
      expect(pipeline.dependsOn).toMatchInlineSnapshot(`
        Array [
          "^build",
        ]
      `);
    });

    it("migrates pipeline with existing env key", () => {
      const config = getTestNxpkgConfig({
        pipeline: { test: { env: ["$MY_ENV"], dependsOn: ["^build"] } },
      });
      const { test } = config.pipeline;
      const pipeline = migratePipeline(test);
      expect(pipeline).toHaveProperty("env");
      expect(pipeline.env).toMatchInlineSnapshot(`
        Array [
          "$MY_ENV",
        ]
      `);
      expect(pipeline.dependsOn).toMatchInlineSnapshot(`
        Array [
          "^build",
        ]
      `);
    });

    it("migrates pipeline with incomplete env key", () => {
      const config = getTestNxpkgConfig({
        pipeline: {
          test: { env: ["$MY_ENV"], dependsOn: ["^build", "$SUPER_COOL"] },
        },
      });
      const { test } = config.pipeline;
      const pipeline = migratePipeline(test);
      expect(pipeline).toHaveProperty("env");
      expect(pipeline.env).toMatchInlineSnapshot(`
        Array [
          "$MY_ENV",
          "SUPER_COOL",
        ]
      `);
      expect(pipeline.dependsOn).toMatchInlineSnapshot(`
        Array [
          "^build",
        ]
      `);
    });

    it("migrates pipeline with duplicate env keys", () => {
      const config = getTestNxpkgConfig({
        pipeline: {
          test: { env: ["$MY_ENV"], dependsOn: ["^build", "$MY_ENV"] },
        },
      });
      const { test } = config.pipeline;
      const pipeline = migratePipeline(test);
      expect(pipeline).toHaveProperty("env");
      expect(pipeline.env).toMatchInlineSnapshot(`
        Array [
          "$MY_ENV",
          "MY_ENV",
        ]
      `);
      expect(pipeline.dependsOn).toMatchInlineSnapshot(`
        Array [
          "^build",
        ]
      `);
    });
  });

  describe("migrateConfig - utility", () => {
    it("migrates config with env var dependencies", () => {
      const config = getTestNxpkgConfig();
      const pipeline = migrateConfig(config);
      expect(pipeline).toMatchInlineSnapshot(`
        Object {
          "$schema": "./docs/public/schema.json",
          "globalEnv": Array [
            "GLOBAL_ENV_KEY",
          ],
          "pipeline": Object {
            "build": Object {
              "dependsOn": Array [
                "^build",
              ],
              "env": Array [
                "TASK_ENV_KEY",
                "ANOTHER_ENV_KEY",
              ],
              "outputs": Array [
                "dist/**/*",
                ".next/**/*",
                "!.next/cache/**",
              ],
            },
            "dev": Object {
              "cache": false,
            },
            "lint": Object {
              "outputs": Array [],
            },
            "test": Object {
              "dependsOn": Array [
                "^build",
              ],
              "outputs": Array [
                "coverage/**/*",
              ],
            },
          },
        }
      `);
    });

    it("migrates config with no env var dependencies", () => {
      const config = getTestNxpkgConfig({
        globalDependencies: [],
        pipeline: {
          build: { dependsOn: ["^build"] },
        },
      });
      const pipeline = migrateConfig(config);
      expect(pipeline).toMatchInlineSnapshot(`
        Object {
          "$schema": "./docs/public/schema.json",
          "pipeline": Object {
            "build": Object {
              "dependsOn": Array [
                "^build",
              ],
              "outputs": Array [
                "dist/**/*",
                ".next/**/*",
                "!.next/cache/**",
              ],
            },
            "dev": Object {
              "cache": false,
            },
            "lint": Object {
              "outputs": Array [],
            },
            "test": Object {
              "dependsOn": Array [
                "^build",
              ],
              "outputs": Array [
                "coverage/**/*",
              ],
            },
          },
        }
      `);
    });

    it("migrates config with inconsistent config", () => {
      const config = getTestNxpkgConfig({
        pipeline: {
          test: { env: ["$MY_ENV"], dependsOn: ["^build", "$SUPER_COOL"] },
        },
      });
      const pipeline = migrateConfig(config);
      expect(pipeline).toMatchInlineSnapshot(`
        Object {
          "$schema": "./docs/public/schema.json",
          "globalEnv": Array [
            "GLOBAL_ENV_KEY",
          ],
          "pipeline": Object {
            "build": Object {
              "dependsOn": Array [
                "^build",
              ],
              "env": Array [
                "TASK_ENV_KEY",
                "ANOTHER_ENV_KEY",
              ],
              "outputs": Array [
                "dist/**/*",
                ".next/**/*",
                "!.next/cache/**",
              ],
            },
            "dev": Object {
              "cache": false,
            },
            "lint": Object {
              "outputs": Array [],
            },
            "test": Object {
              "dependsOn": Array [
                "^build",
              ],
              "env": Array [
                "$MY_ENV",
                "SUPER_COOL",
              ],
              "outputs": Array [
                "coverage/**/*",
              ],
            },
          },
        }
      `);
    });

    it("migrates config with duplicate env keys", () => {
      const config = getTestNxpkgConfig({
        pipeline: {
          test: { env: ["$MY_ENV"], dependsOn: ["^build", "$MY_ENV"] },
        },
      });
      const pipeline = migrateConfig(config);
      expect(pipeline).toMatchInlineSnapshot(`
        Object {
          "$schema": "./docs/public/schema.json",
          "globalEnv": Array [
            "GLOBAL_ENV_KEY",
          ],
          "pipeline": Object {
            "build": Object {
              "dependsOn": Array [
                "^build",
              ],
              "env": Array [
                "TASK_ENV_KEY",
                "ANOTHER_ENV_KEY",
              ],
              "outputs": Array [
                "dist/**/*",
                ".next/**/*",
                "!.next/cache/**",
              ],
            },
            "dev": Object {
              "cache": false,
            },
            "lint": Object {
              "outputs": Array [],
            },
            "test": Object {
              "dependsOn": Array [
                "^build",
              ],
              "env": Array [
                "$MY_ENV",
                "MY_ENV",
              ],
              "outputs": Array [
                "coverage/**/*",
              ],
            },
          },
        }
      `);
    });
  });

  describe("transform", () => {
    const { useFixture } = setupTestFixtures({
      directory: __dirname,
      test: "migrate-env-var-dependencies",
    });

    it("migrates nxpkg.json env var dependencies - basic", () => {
      // load the fixture for the test
      const { root, read } = useFixture({
        fixture: "env-dependencies",
      });

      // run the transformer
      const result = transformer({
        root,
        options: { force: false, dry: false, print: false },
      });

      expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
        $schema: "https://nxpkg.build/schema.json",
        globalDependencies: [".env"],
        globalEnv: ["NEXT_PUBLIC_API_KEY", "STRIPE_API_KEY"],
        pipeline: {
          build: {
            dependsOn: ["^build"],
            env: ["PROD_API_KEY"],
            outputs: [".next/**", "!.next/cache/**"],
          },
          dev: {
            cache: false,
          },
          lint: {
            dependsOn: [],
            env: ["IS_CI"],
            outputs: [],
          },
          test: {
            dependsOn: ["test"],
            env: ["IS_CI"],
            outputs: [],
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

    it("migrates nxpkg.json env var dependencies - workspace configs", () => {
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
        globalDependencies: [".env"],
        globalEnv: ["NEXT_PUBLIC_API_KEY", "STRIPE_API_KEY"],
        pipeline: {
          build: {
            dependsOn: ["^build"],
            env: ["PROD_API_KEY"],
            outputs: [".next/**", "!.next/cache/**"],
          },
          dev: {
            cache: false,
          },
          lint: {
            dependsOn: [],
            env: ["IS_TEST"],
            outputs: [],
          },
          test: {
            dependsOn: ["test"],
            env: ["IS_CI"],
            outputs: [],
          },
        },
      });

      expect(readJson("apps/web/nxpkg.json") || "{}").toStrictEqual({
        $schema: "https://nxpkg.build/schema.json",
        extends: ["//"],
        pipeline: {
          build: {
            // old
            dependsOn: ["build"],
            // new
            env: ["ENV_1", "ENV_2"],
          },
        },
      });

      expect(readJson("packages/ui/nxpkg.json") || "{}").toStrictEqual({
        $schema: "https://nxpkg.build/schema.json",
        extends: ["//"],
        pipeline: {
          build: {
            dependsOn: [],
            env: ["IS_SERVER"],
          },
        },
      });

      expect(result.fatalError).toBeUndefined();
      expect(result.changes).toMatchInlineSnapshot(`
        Object {
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
            "additions": 4,
            "deletions": 4,
          },
        }
      `);
    });

    it("migrates nxpkg.json env var dependencies - repeat run", () => {
      // load the fixture for the test
      const { root, read } = useFixture({
        fixture: "env-dependencies",
      });

      // run the transformer
      const result = transformer({
        root,
        options: { force: false, dry: false, print: false },
      });

      expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
        $schema: "https://nxpkg.build/schema.json",
        globalDependencies: [".env"],
        globalEnv: ["NEXT_PUBLIC_API_KEY", "STRIPE_API_KEY"],
        pipeline: {
          build: {
            dependsOn: ["^build"],
            env: ["PROD_API_KEY"],
            outputs: [".next/**", "!.next/cache/**"],
          },
          dev: {
            cache: false,
          },
          lint: {
            dependsOn: [],
            env: ["IS_CI"],
            outputs: [],
          },
          test: {
            dependsOn: ["test"],
            env: ["IS_CI"],
            outputs: [],
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

      // run the transformer
      const repeatResult = transformer({
        root,
        options: { force: false, dry: false, print: false },
      });

      expect(repeatResult.fatalError).toBeUndefined();
      expect(repeatResult.changes).toMatchInlineSnapshot(`
        Object {
          "nxpkg.json": Object {
            "action": "unchanged",
            "additions": 0,
            "deletions": 0,
          },
        }
      `);
    });

    it("migrates nxpkg.json env var dependencies - dry", () => {
      // load the fixture for the test
      const { root, read } = useFixture({
        fixture: "env-dependencies",
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
            "additions": 4,
            "deletions": 4,
          },
        }
      `);
    });

    it("migrates nxpkg.json env var dependencies - print", () => {
      // load the fixture for the test
      const { root, read } = useFixture({
        fixture: "env-dependencies",
      });

      // run the transformer
      const result = transformer({
        root,
        options: { force: false, dry: false, print: true },
      });

      expect(JSON.parse(read("nxpkg.json") || "{}")).toStrictEqual({
        $schema: "https://nxpkg.build/schema.json",
        globalEnv: ["NEXT_PUBLIC_API_KEY", "STRIPE_API_KEY"],
        globalDependencies: [".env"],
        pipeline: {
          build: {
            dependsOn: ["^build"],
            env: ["PROD_API_KEY"],
            outputs: [".next/**", "!.next/cache/**"],
          },
          dev: {
            cache: false,
          },
          lint: {
            dependsOn: [],
            env: ["IS_CI"],
            outputs: [],
          },
          test: {
            dependsOn: ["test"],
            env: ["IS_CI"],
            outputs: [],
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

    it("migrates nxpkg.json env var dependencies - dry & print", () => {
      // load the fixture for the test
      const { root, read } = useFixture({
        fixture: "env-dependencies",
      });

      const nxpkgJson = JSON.parse(read("nxpkg.json") || "{}") as Schema;

      // run the transformer
      const result = transformer({
        root,
        options: { force: false, dry: true, print: true },
      });

      // make sure it didn't change
      expect(JSON.parse(read("nxpkg.json") || "{}")).toEqual(nxpkgJson);

      expect(result.fatalError).toBeUndefined();
      expect(result.changes).toMatchInlineSnapshot(`
        Object {
          "nxpkg.json": Object {
            "action": "skipped",
            "additions": 4,
            "deletions": 4,
          },
        }
      `);
    });

    it("does not change nxpkg.json if already migrated", () => {
      // load the fixture for the test
      const { root, read } = useFixture({
        fixture: "migrated-env-dependencies",
      });

      const nxpkgJson = JSON.parse(read("nxpkg.json") || "{}") as Schema;

      // run the transformer
      const result = transformer({
        root,
        options: { force: false, dry: false, print: false },
      });

      expect(JSON.parse(read("nxpkg.json") || "{}")).toEqual(nxpkgJson);

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
});
