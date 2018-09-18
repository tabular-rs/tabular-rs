use column_spec::{ColumnSpec, parse_row_spec, row_spec_to_string};
use error::Result;
use row::{InternalRow, Row};

use std::fmt::{Debug, Formatter, Display};

/// Builder type for constructing a formatted table.
///
/// Construct this with [`Table::new()`] or [`Table::new_safe()`]. Then add rows
/// to it with [`Table::add_row()`] and [`Table::add_heading()`].
///
/// [`Table::new_safe()`]: struct.Table.html#method.new_safe
/// [`Table::new()`]: struct.Table.html#method.new
/// [`Table::add_row()`]: struct.Table.html#method.add_row
/// [`Table::add_heading()`]: struct.Table.html#method.add_heading
#[derive(Clone)]
pub struct Table {
    n_columns:     usize,
    format:        Vec<ColumnSpec>,
    rows:          Vec<InternalRow>,
    column_widths: Vec<usize>,
}

impl Table {
    /// Constructs a new table with the format of each row specified by `row_spec`.
    ///
    /// Unlike `format!` and friends, `row_spec` is processed manually, but it uses a small
    /// subset of the syntax to determine how columns are laid out. In particular:
    ///
    ///   - `{:<}` produces a left-aligned column.
    ///
    ///   - `{:>}` produces a right-aligned column.
    ///
    ///   - `{{` produces a literal `{` character.
    ///
    ///   - `}}` produces a literal `}` character.
    ///
    ///   - Everything else stands for itself.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tabular::*;
    /// let table = Table::new("{{:<}} produces ‘{:<}’ and {{:>}} produces ‘{:>}’")
    ///     .with_row(Row::from_cells(["a", "bc"].iter().cloned()));
    /// ```
    pub fn new(row_spec: &str) -> Self {
        Self::new_safe(row_spec).unwrap_or_else(|e: super::error::Error|
            panic!("tabular::Table::new: {}", e))
    }

    /// Like [`new`], but returns a [`Result`] instead of panicking if parsing `row_spec` fails.
    ///
    /// [`new`]: #method.new
    /// [`Result`]: type.Result.html
    pub fn new_safe(row_spec: &str) -> Result<Self> {
        let (format, n_columns) = parse_row_spec(row_spec)?;
        Ok(Table {
            n_columns,
            format,
            rows:           vec![],
            column_widths:  vec![0; n_columns]
        })
    }

    /// The number of columns in the table.
    pub fn column_count(&self) -> usize {
        // ^^^^^^^^^^^^ What’s a better name for this?
        self.n_columns
    }

    /// Adds a pre-formatted row that spans all columns.
    ///
    /// A heading does not interact with the formatting of rows made of cells.
    /// This is like `\intertext` in LaTeX, not like `<head>` or `<th>` in HTML.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tabular::*;
    ///         let mut table = Table::new("{:<}  {:>}");
    ///         table
    ///             .add_heading("./:")
    ///             .add_row(Row::new().with_cell("Cargo.lock").with_cell(433))
    ///             .add_row(Row::new().with_cell("Cargo.toml").with_cell(204))
    ///             .add_heading("")
    ///             .add_heading("src/:")
    ///             .add_row(Row::new().with_cell("lib.rs").with_cell(10257))
    ///             .add_heading("")
    ///             .add_heading("target/:")
    ///             .add_row(Row::new().with_cell("debug/").with_cell(672));
    /// 
    ///         assert_eq!( format!("{}", table),
    ///                     "./:\n\
    ///                      Cargo.lock    433\n\
    ///                      Cargo.toml    204\n\
    ///                      \n\
    ///                      src/:\n\
    ///                      lib.rs      10257\n\
    ///                      \n\
    ///                      target/:\n\
    ///                      debug/        672\n\
    ///                      " );
    /// ```
    /// 
    pub fn add_heading<S: Into<String>>(&mut self, heading: S) -> &mut Self {
        self.rows.push(InternalRow::Heading(heading.into()));
        self
    }

    /// Convenience function for calling [`add_heading`].
    ///
    /// [`add_heading`]: #method.add_heading
    pub fn with_heading<S: Into<String>>(mut self, heading: S) -> Self {
        self.add_heading(heading);
        self
    }

    /// Adds a row made up of cells.
    ///
    /// When printed, each cell will be padded to the size of its column, which is the maximum of
    /// the width of its cells.
    ///
    /// # Panics
    ///
    /// If `self.`[`column_count()`]` != row.`[`len()`].
    ///
    /// [`column_count()`]: #method.column_count
    /// [`len()`]: struct.Row.html#method.len
    pub fn add_row(&mut self, row: Row) -> &mut Self {
        let cells = row.0;

        assert_eq!(cells.len(), self.n_columns);

        for (width, s) in self.column_widths.iter_mut().zip(cells.iter()) {
            *width = ::std::cmp::max(*width, s.width());
        }

        self.rows.push(InternalRow::Cells(cells));
        self
    }

    /// Convenience function for calling [`add_row`].
    ///
    /// # Panics
    ///
    /// The same as [`add_row`].
    ///
    /// [`add_row`]: #method.add_row
    pub fn with_row(mut self, row: Row) -> Self {
        self.add_row(row);
        self
    }
}

impl Debug for Table {
    // This method allocates in two places:
    //   - row_spec_to_string
    //   - row.clone()
    // It doesn't need to do either.
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "Table::new({:?})", row_spec_to_string(&self.format))?;

        for row in &self.rows {
            match *row {
                InternalRow::Cells(ref row) => {
                    write!(f, ".with_row({:?})", Row(row.clone()))?
                },

                InternalRow::Heading(ref heading) => {
                    write!(f, ".with_heading({:?})", heading)?
                },
            }
        }

        Ok(())
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        use column_spec::ColumnSpec::*;

        let max_column_width = self.column_widths.iter().cloned().max().unwrap_or(0);
        let mut spaces = String::with_capacity(max_column_width);
        for _ in 0 .. max_column_width {
            spaces.push(' ');
        }

        for row in &self.rows {
            match row {
                InternalRow::Cells(cells) => {
                    let mut cw_iter  = self.column_widths.iter().cloned();
                    let mut row_iter = cells.iter();

                    for field in 0 .. self.format.len() {
                        let fs = &self.format[field];

                        match fs {
                            Left  => {
                                let cw    = cw_iter.next().unwrap();
                                let width = match row_iter.next() {
                                    Some(ws) => {
                                        f.write_str(ws.as_str())?;
                                        ws.width()
                                    }
                                    None     => 0,
                                };

                                if field + 1 < self.format.len() {
                                    f.write_str(&spaces[.. cw - width])?;
                                }
                            }

                            Right => {
                                let cw = cw_iter.next().unwrap();
                                match row_iter.next() {
                                    Some(ws) => {
                                        f.write_str(&spaces[.. cw - ws.width()])?;
                                        f.write_str(ws.as_str())?;
                                    },
                                    None     => f.write_str(&spaces[.. cw])?,
                                };
                            }

                            Literal(s) => f.write_str(&s)?,
                        }
                    }
                }

                InternalRow::Heading(s) => {
                    f.write_str(s)?;
                }
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

