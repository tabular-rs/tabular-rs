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
    pub fn new(row_spec: &str) -> Self {
        Self::new_safe(row_spec).unwrap_or_else(|e|
            panic!("tabular::Table::new: {}", e))
    }

    pub fn new_safe(row_spec: &str) -> Result<Self> {
        let (format, n_columns) = parse_row_spec(row_spec)?;
        Ok(Table {
            n_columns,
            format,
            rows:           vec![],
            column_widths:  vec![0; n_columns]
        })
    }

    pub fn add_heading<S: Into<String>>(&mut self, heading: S) -> &mut Self {
        self.rows.push(InternalRow::Heading(heading.into()));
        self
    }

    pub fn with_heading<S: Into<String>>(mut self, heading: S) -> Self {
        self.add_heading(heading);
        self
    }

    pub fn add_row(&mut self, row: Row) -> &mut Self {
        let cells = row.0;

        assert_eq!(cells.len(), self.n_columns);

        for (width, s) in self.column_widths.iter_mut().zip(cells.iter()) {
            *width = ::std::cmp::max(*width, s.width());
        }

        self.rows.push(InternalRow::Cells(cells));
        self
    }

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

