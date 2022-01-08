# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html

## [0.2.0] - 2022-01-08

### Added
- `Row::with_ansi_cell` and `add_ansi_cell` add cells that potentially have
  ANSI color codes in them; the color codes are stripped out while computing
  the width.
- `Row::with_custom_width_cell` and `add_custom_width_cell` add cells with a
  custom width.

### Changed
- Methods `Row::with_cell`, `Table::width_heading`,
`Table::with_row`, and `Table::set_line_end` now have the
`#[must_use]` attribute.
- MSRV bumped to 1.46.

## [0.1.4] - 2019-12-29

### Changed
- Oldest supported Rust version is now 1.31.0.

## [0.1.3] - 2019-12-29

### Changed
- No longer depends on deprecated `str::trim_right` method.

## [0.1.2] - 2018-09-18

### Added
- `Table::set_line_end()` method for changing the line ending used by
formatted tables.
- `row!()` and `table!()` macros.

### Changed
- Centering now rounds to left rather than right; I think it looks better.

## [0.1.1] - 2018-09-18

### Added
- `{:^}` column specifier for centering.

## [0.1.0] - 2019/09/18

Initial release.

[0.1.0]: https://github.com/tabular-rs/tabular-rs/releases/tag/0.1.0
[0.1.1]: https://github.com/tabular-rs/tabular-rs/releases/tag/0.1.1
[0.1.2]: https://github.com/tabular-rs/tabular-rs/releases/tag/0.1.2
[0.1.3]: https://github.com/tabular-rs/tabular-rs/releases/tag/0.1.3
[0.1.4]: https://github.com/tabular-rs/tabular-rs/releases/tag/0.1.4
[0.2.0]: https://github.com/tabular-rs/tabular-rs/releases/tag/0.2.0
