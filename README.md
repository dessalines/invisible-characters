# Invisible-Characters

A rust crate which exports an array of invisible characters. You can use this list with either a character or regex checker.

The chars are sourced from [flopp/invisible-characters](https://github.com/flopp/invisible-characters).

Usage:

`cargo add invisible-characters`

```rust
use invisible_characters::INVISIBLE_CHARS;

if title.contains(INVISIBLE_CHARS) {...}
```
