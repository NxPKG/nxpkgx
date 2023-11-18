# `eslint-plugin-nxpkg`

Ease configuration for Nxpkgrepo

## Installation

1. You'll first need to install [ESLint](https://eslint.org/):

```sh
npm install eslint --save-dev
```

2. Next, install `eslint-plugin-nxpkg`:

```sh
npm install eslint-plugin-nxpkg --save-dev
```

## Usage

Add `nxpkg` to the plugins section of your `.eslintrc` configuration file. You can omit the `eslint-plugin-` prefix:

```json
{
  "plugins": ["nxpkg"]
}
```

Then configure the rules you want to use under the rules section.

```json
{
  "rules": {
    "nxpkg/no-undeclared-env-vars": "error"
  }
}
```

### Example

```json
{
  "plugins": ["nxpkg"],
  "rules": {
    "nxpkg/no-undeclared-env-vars": [
      "error",
      {
        "allowList": ["^ENV_[A-Z]+$"]
      }
    ]
  }
}
```
