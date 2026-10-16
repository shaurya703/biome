---
"@biomejs/biome": patch
---

Fixed [#12374](https://github.com/biomejs/biome/issues/12374): `biome migrate eslint` now carries the `null` handling of ESLint's `eqeqeq` rule over to [`noDoubleEquals`](https://biomejs.dev/linter/rules/no-double-equals/).

ESLint's default (`"always"`) reports `== null`, whereas Biome ignores it by default. The migration now sets `ignoreNull: false` for `"always"` and `ignoreNull: true` for `"smart"`, `"allow-null"`, and `{ "null": "ignore" }`.
