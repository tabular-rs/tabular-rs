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
}
