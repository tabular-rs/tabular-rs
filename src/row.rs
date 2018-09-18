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
    pub fn new() -> Self {
        Row(Vec::new())
    }

    pub fn add_cell<S: Display>(&mut self, value: S) -> &mut Self {
        self.0.push(WidthString::new(value));
        self
    }

    pub fn with_cell<S: Display>(mut self, value: S) -> Self {
        self.add_cell(value);
        self
    }

    pub fn from_cells<S, I>(values: I) -> Self
        where S: Into<String>,
              I: IntoIterator<Item = S> {

        Row(values.into_iter().map(Into::into).map(WidthString::new).collect())
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

