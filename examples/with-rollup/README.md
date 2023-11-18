# Nxpkgrepo starter with Rollup

This is an official starter Nxpkgrepo, showing how Nxpkgrepo can be used with Rollup for bundling a `ui` package.

## Using this example

Run the following command:

```sh
npx create-nxpkg@latest -e with-rollup
```

## What's inside?

This Nxpkgrepo includes the following packages/apps:

### Apps and Packages

- `ui`: a React component library used by the `web` application, compiled with Rollup
- `web`: a [Next.js](https://nextjs.org) app
- `eslint-config-custom`: `eslint` configurations (includes `eslint-config-next` and `eslint-config-prettier`)
- `tsconfig`: `tsconfig.json`s used throughout the monorepo

Each package/app is 100% [TypeScript](https://www.typescriptlang.org/).

### Utilities

This Nxpkgrepo has some additional tools already setup for you:

- [TypeScript](https://www.typescriptlang.org/) for static type checking
- [ESLint](https://eslint.org/) for code linting
- [Prettier](https://prettier.io) for code formatting

### Build

To build all apps and packages, run the following command:

```
cd my-nxpkgrepo
pnpm run build
```

### Develop

To develop all apps and packages, run the following command:

```
cd my-nxpkgrepo
pnpm run dev
```

### Remote Caching

Nxpkgrepo can use a technique known as [Remote Caching](https://nxpkgrepo.org/docs/core-concepts/remote-caching) to share cache artifacts across machines, enabling you to share build caches with your team and CI/CD pipelines.

By default, Nxpkgrepo will cache locally. To enable Remote Caching you will need an account with Vercel. If you don't have an account you can [create one](https://vercel.com/signup), then enter the following commands:

```
cd my-nxpkgrepo
npx nxpkg login
```

This will authenticate the Nxpkgrepo CLI with your [Vercel account](https://vercel.com/docs/concepts/personal-accounts/overview).

Next, you can link your Nxpkgrepo to your Remote Cache by running the following command from the root of your Nxpkgrepo:

```
npx nxpkg link
```

## Useful Links

Learn more about the power of Nxpkgrepo:

- [Pipelines](https://nxpkgrepo.org/docs/core-concepts/pipelines)
- [Caching](https://nxpkgrepo.org/docs/core-concepts/caching)
- [Remote Caching](https://nxpkgrepo.org/docs/core-concepts/remote-caching)
- [Scoped Tasks](https://nxpkgrepo.org/docs/core-concepts/scopes)
- [Configuration Options](https://nxpkgrepo.org/docs/reference/configuration)
- [CLI Usage](https://nxpkgrepo.org/docs/reference/command-line-reference)
