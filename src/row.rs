use super::width_string::WidthString;

use std::fmt::{Debug, Display, Formatter};

/// Type for building a [`Table`] row.
///
/// Make a new one with [`Row::new()`], then add to it with [`Row::with_cell()`].
/// Or make a complete one with [`Row::from_cells()`].
///
/// [`Table`]: struct.Table.html
/// [`Row::new()`]: struct.Row.html#method.new
/// [`Row::from_cells()`]: struct.Row.html#method.from_cells
/// [`Row::with_cell()`]: struct.Row.html#method.with_cell
#[derive(Clone, Default)]
pub struct Row(pub (crate) Vec<WidthString>);

impl Row {
    /// Makes a new, empty table row.
    pub fn new() -> Self {
        Row(Vec::new())
    }

    /// Adds a cell to this table row.
    ///
    /// # Examples
    ///
    /// ```
    /// struct DirEntry {
    ///     size:         usize,
    ///     is_directory: bool,
    ///     name:         String,
    /// }
    ///
    /// impl DirEntry {
    ///     fn to_row(&self) -> tabular::Row {
    ///         tabular::Row::new()
    ///             .with_cell(self.size)
    ///             .with_cell(if self.is_directory { "d" } else { "" })
    ///             .with_cell(&self.name)
    ///     }
    /// }
    /// ```
    pub fn with_cell<S: Display>(mut self, value: S) -> Self {
        self.add_cell(value);
        self
    }

    /// Adds a cell to this table row.
    pub fn add_cell<S: Display>(&mut self, value: S) -> &mut Self {
        self.0.push(WidthString::new(value));
        self
    }

    pub fn from_cells<S, I>(values: I) -> Self
        where S: Into<String>,
              I: IntoIterator<Item = S> {

        Row(values.into_iter().map(Into::into).map(WidthString::new).collect())
    }
    
    /// The number of cells in this row.
    ///
    /// # Examples
    ///
    /// It's probably not actually useful, because you are unlikely to come
    /// upon a row whose size you don't already know. But it's useful for stating
    /// [`Table::add_row`]'s invariant.
    ///
    /// ```
    /// # use tabular::*;
    /// fn print_ragged_matrix<T: ::std::fmt::Display>(matrix: &[&[T]]) {
    ///    let ncols = matrix.iter().map(|row| row.len()).max().unwrap_or(0);
    ///
    ///    let mut row_spec = String::with_capacity(5 * ncols);
    ///    for _ in 0 .. ncols {
    ///        row_spec.push_str("{:>} ");
    ///    }
    ///
    ///    let mut table = Table::new(row_spec.trim_right());
    ///
    ///    for row in matrix {
    ///        let mut table_row = Row::from_cells(row.iter().map(ToString::to_string));
    ///
    ///        // Don't remember how to count or subtract but I'll get there eventually.
    ///        while table_row.len() < table.column_count() {
    ///            table_row.add_cell("");
    ///        }
    ///    }
    ///
    ///    print!("{}", table);
    /// }
    ///
    /// print_ragged_matrix(&[&[1, 2, 3, 4, 5], &[12, 23, 34], &[123, 234], &[1234]]);
    /// ```
    ///
    /// [`Table::add_row`]: struct.Table.html#method.add_row
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Debug for Row {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "Row::from_cells(vec!{:?})", self.0)
    }
}

#[derive(Clone, Debug)]
pub enum InternalRow {
    Cells(Vec<WidthString>),
    Heading(String),
}

