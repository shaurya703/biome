---
"@biomejs/biome": patch
---

Fixed [#12373](https://github.com/biomejs/biome/issues/12373): `biome migrate eslint` now only migrates ESLint's `one-var` rule to [`useSingleVarDeclarator`](https://biomejs.dev/linter/rules/use-single-var-declarator/) when it is configured with `"never"`, which is the only option the Biome rule enforces.

The default `"always"`, `"consecutive"`, and object options that are not all `"never"` require combining declarations, so the migration now reports the rule as not implemented instead of enabling a rule that enforces the opposite.
