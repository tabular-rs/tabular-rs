# tabular: plain text tables in Rust

[![Build Status](https://travis-ci.org/tov/tabular-rs.svg?branch=master)](https://travis-ci.org/tov/tabular-rs)
[![Crates.io](https://img.shields.io/crates/v/tabular.svg?maxAge=2592000)](https://crates.io/crates/tabular)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](LICENSE-APACHE)

Builds plain, automatically-aligned tables of monospaced text.
This is basically what you want if you are implementing `ls`.

## Example

```rust
use console::style;
use tabular::{Table, Row};
use std::path::Path;

struct TerminalColored(String);

impl From<TerminalColored(String)> for WidthString {
    fn from(c: TerminalColored(String)) -> WidthString {
        let styled = style(c.0.as_str()).cyan();
        Self::new(styled.as_str(), console::measure_text_width(styled))
    }
}

fn ls(dir: &Path) -> ::std::io::Result<()> {
    let mut table = Table::new("{:>}  {:<}{:<}  {:<}");
    for entry_result in ::std::fs::read_dir(dir)? {
        let entry    = entry_result?;
        let metadata = entry.metadata()?;

        table.add_row(Row::new()
             .with_cell(metadata.len())
             .with_cell(if metadata.permissions().readonly() {"r"} else {""})
             .with_cell(if metadata.is_dir() {"d"} else {""})
             .with_cell(entry.path().display()));
    }

    print!("{}", table);

    Ok(())
}

ls(Path::new(&"target")).unwrap();
```

produces something like

```
1198     target/.rustc_info.json
1120  d  target/doc
 192  d  target/package
1056  d  target/debug
```

## Other features

  - The `Table::with_header()` and `Table::add_header()` methods add
    lines that span all columns.

  - The `row!` macro builds a row with a fixed number of columns
    using less syntax.

  - The `Table::set_line_end()` method allows changing the line ending
    to include a carriage return (or whatever you want).


## Usage

It's on [crates.io](https://crates.io/crates/tabular), so you can add

```toml
[dependencies]
tabular = "0.1.4"
```

to your `Cargo.toml`.

Feature `unicode-width` is enabled be default; it depends on the
[unicode-width](https://crates.io/crates/unicode-width) crate. You can turn 
it off with:

```toml
[dependencies]
tabular = { version = "0.1.4", default-features = false }
```

Note that without `unicode-width`, alignment will be based on the count of the
`std::str::Chars` iterator.

This crate supports Rust version 1.31.0 and later.

## See also

You may also want:

- [text-tables](https://crates.io/crates/text-tables) – This is more automatic
  than tabular. You give it an array of arrays, it renders a nice table with 
  borders. Tabular doesn't do borders.

- [prettytable](https://crates.io/crates/prettytable-rs) — This has an API more
  similar to tabular’s in terms of building a table, but it does a lot more, 
  including, color, borders, and CSV import.
