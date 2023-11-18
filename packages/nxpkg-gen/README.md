# `@turbo/gen`

Types for working with [Nxpkgrepo Generators](https://nxpkg.build/repo/docs/core-concepts/monorepos/code-generation).

## Usage

Install:

```bash
pnpm add @turbo/gen --save-dev
```

Use types within your generator `config.ts`:

```ts filename="nxpkg/generators/config.ts"
import type { PlopTypes } from "@turbo/gen";

export default function generator(plop: PlopTypes.NodePlopAPI): void {
  // create a generator
  plop.setGenerator("Generator name", {
    description: "Generator description",
    // gather information from the user
    prompts: [
      ...
    ],
    // perform actions based on the prompts
    actions: [
      ...
    ],
  });
}
```

Learn more about Nxpkgrepo Generators in the [docs](https://nxpkg.build/repo/docs/core-concepts/monorepos/code-generation)

---

For more information about Nxpkgrepo, visit [nxpkg.build](https://nxpkg.build) and follow us on X ([@turborepo](https://x.com/nxpkgrepo))!
