#![doc(html_root_url = "https://docs.rs/tabular/0.1.0")]
//! Builds plain, automatically-aligned tables of monospaced text.
//!
//! This is basically what you if you are implementing `ls`.
//!
//! # Usage
//!
//! The number and alignment of the columns is determined by a format string
//! passed to [`Table::new`]. Then, [`Row`]s are added to the [`Table`]
//! using [`Table::add_row`] or [`Table::with_row`]. Each row is is constructed
//! by [`Row::new`] or `[Row::from_cells]`
//!
//! ## Getting it
//! 
//! It's on [crates.io](https://crates.io/crates/tabular), so you can add
//!
//! ```toml
//! [dependencies]
//! tabular = "0.1.0"
//! ```
//!
//! to your `Cargo.toml`.
//!
//! [`Row`]: struct.Row.html
//! [`Table`]: struct.Table.html
//! [`Table::add_row`]: struct.Table.html#method.add_row
//! [`Table::new`]: struct.Table.html#method.new
//! [`Table::with_row`]: struct.Table.html#method.with_row

#![warn(missing_docs)]

#[cfg(feature = "unicode-width")]
extern crate unicode_width;

mod column_spec;
mod error;
mod row;
mod table;
mod width_string;

pub use error::{Error, Result};
pub use row::Row;
pub use table::Table;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment() {
        let mut table = Table::new("{:>}  ({:<}) {:<}");
        table
            .add_row(Row::new().with_cell(1).with_cell("I").with_cell("one"))
            .add_row(Row::new().with_cell(5).with_cell("V").with_cell("five"))
            .add_row(Row::new().with_cell(10).with_cell("X").with_cell("ten"))
            .add_row(Row::new().with_cell(50).with_cell("L").with_cell("fifty"))
            .add_row(Row::new().with_cell(100).with_cell("C").with_cell("one-hundred"));
        assert_eq!( format!("\n{}", table),
                    r#"
  1  (I) one
  5  (V) five
 10  (X) ten
 50  (L) fifty
100  (C) one-hundred
"# );
    }

    #[test]
    fn heading() {
        let _row = Row::from_cells(vec!["a", "b", "c"]);
//        eprintln!("{:?}", _row);

        let table = Table::new("{:<} {:<} {:>}")
            .with_row(Row::from_cells(vec!["a", "b", "d"]))
            .with_heading("This is my table")
            .with_row(Row::from_cells(vec!["ab", "bc", "cd"]));

//        eprintln!("\n\n{:?}\n\n", table);

        assert_eq! ( format!("\n{}", table),
                     r#"
a  b   d
This is my table
ab bc cd
"# );
    }

    #[test]
    fn temporary() {
      
    }
}
