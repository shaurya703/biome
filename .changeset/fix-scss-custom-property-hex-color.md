---
"@biomejs/biome": patch
---

Fixed [#12332](https://github.com/biomejs/biome/issues/12332): a hexadecimal colour that starts with a digit, such as `#4566a8`, is now parsed as a colour inside a SCSS custom property value. It used to be split into `#` and a dimension, so `noUnknownUnit` reported the colour's tail (`a8`) as an unknown unit.
