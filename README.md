# tabular: plain text tables in Rust

[![Build Status]][CI]
[![Crates.io]][crate]
[![License: MIT]](LICENSE-MIT)
[![License: Apache 2.0]](LICENSE-APACHE)
[![Documentation (latest release)]](https://docs.rs/tabular/latest)

[Build Status]:
  <https://github.com/tov/tabular-rs/actions/workflows/ci.yml/badge.svg>  

[CI]:
  <https://github.com/tov/tabular-rs/actions>

[Crates.io]:
  <https://img.shields.io/crates/v/tabular.svg?maxAge=2592000>

[crate]:
  <https://crates.io/crates/tabular>

[License: MIT]:
  <https://img.shields.io/badge/license-MIT-blue.svg>

[License: Apache 2.0]:
  <https://img.shields.io/badge/license-Apache_2.0-blue.svg>

[Documentation (latest release)]:
  <https://img.shields.io/docsrs/tabular.svg>

Builds plain, automatically-aligned tables of monospaced text.
This is basically what you want if you are implementing `ls`.

## Example

```rust
use tabular::{Table, Row};
use std::path::Path;

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

  - The `Table::with_heading()` and `Table::add_heading()` methods add
    lines that span all columns.

  - The `row!` macro builds a row with a fixed number of columns
    using less syntax.

  - The `Table::set_line_end()` method allows changing the line ending
    to include a carriage return (or whatever you want).

  - With the `ansi-cell` feature enabled, the `Row::with_ansi_cell` and `Row::add_ansi_cell` methods can be
    used to add cells with ANSI color codes, and still have their widths be
    computed correctly.

  - The `Row::with_custom_width_cell` and `Row::add_custom_width_cell` methods
    can be used to customize alignment precisely.

## Usage

It's on [crates.io](https://crates.io/crates/tabular), so you can add

```toml
[dependencies]
tabular = "0.1.4"
```

to your `Cargo.toml`.

## Features

* `unicode-width`: enabled by default; depends on the
[unicode-width](https://crates.io/crates/unicode-width) crate.

    With the `unicode-width` feature enabled, default alignment is based on [Unicode Standard Annex #11], with characters in the Ambiguous category considered to be 1 column wide.

    Without it, default alignment is based on the count of the `std::str::Chars` iterator.

* `ansi-cell`: disabled by default; depends on the [strip-ansi-escapes](https://crates.io/crates/strip-ansi-escapes) crate. Provides the `with_ansi_cell` and `add_ansi_cell` methods.

## Minimum supported Rust version

The minimum supported Rust version (MSRV) of this crate is **Rust 1.46.0**.
The MSRV may be bumped in a patch release.

## See also

You may also want:

- [text-tables](https://crates.io/crates/text-tables) – This is more automatic
  than tabular. You give it an array of arrays, it renders a nice table with 
  borders. Tabular doesn't do borders.

- [prettytable](https://crates.io/crates/prettytable-rs) — This has an API more
  similar to tabular’s in terms of building a table, but it does a lot more, 
  including, color, borders, and CSV import.
