---
"@biomejs/biome": patch
---

Improved the performance of [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/), [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/), and the other lint rules that check whether a name is used in another language of the same file. In files that contain a single language, such as plain JavaScript and TypeScript files, these rules no longer do a database lookup for every name.
