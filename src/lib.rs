#[cfg(feature = "unicode-width")]
extern crate unicode_width;

use std::fmt::{Debug, Formatter, Display};

/// Errors from parsing the table format string.
///
/// Returned by [`Table::new_safe()`].
///
/// [`Table::new_safe()`]: struct.Table.html#method.new_safe
#[derive(Debug, Clone)]
pub enum Error {
    /// Encountered a `{` character without matching `}`.
    ///
    /// The string is the contents of the column specifier, not including the `{` character.
    UnclosedColumnSpec(String),
    /// Did not understand the column specifiier.
    ///
    /// The string is the contents of the column specifier, not including the `{`
    /// and `}` characters.
    BadColumnSpec(String),
    /// Encountered a `}` character without a prior matching `{` character.
    UnexpectedRightBrace,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnclosedColumnSpec(_) => "unclosed column specifier",
            Error::BadColumnSpec(_) => "bad format specifier",
            Error::UnexpectedRightBrace => "unexpected single ‘}’ character",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            Error::UnclosedColumnSpec(ref spec) =>
                write!(f, "unclosed column specifier: ‘{{{}’", spec),
            Error::BadColumnSpec(ref spec) =>
                write!(f, "bad format specifier: ‘{{{}}}’", spec),
            Error::UnexpectedRightBrace =>
                f.write_str("unexpected single ‘}’ character"),
        }
    }
}

/// Type alias specializing `std::result::Result` with this crate’s [`Error`] enum.
///
/// [`Error`]: error.Error.html
pub type Result<T> = std::result::Result<T, Error>;

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
pub struct Row(Vec<WidthString>);

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
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Row::from_cells(vec!{:?})", self.0)
    }
}

#[derive(Clone, Debug)]
enum FormatSpec {
    Left,
    Right,
    Literal(String),
}

fn format_string_to_string(specs: &[FormatSpec]) -> String {
    use self::FormatSpec::*;

    let mut result = String::new();

    for spec in specs {
        match *spec {
            Left  => result.push_str("{:<}"),
            Right => result.push_str("{:>}"),
            Literal(ref literal) => {
                for c in literal.chars() {
                    match c {
                        '{' => result.push_str("{{"),
                        '}' => result.push_str("}}"),
                        _   => result.push(c),
                    }
                }
            }
        }
    }

    result
}

fn get_column_spec(chars: &mut std::str::Chars) -> Result<String> {
    let mut result = String::new();

    while let Some(c) = chars.next() {
        if c == '}' {
            return Ok(result);
        }

        result.push(c);
    }

    Err(Error::UnclosedColumnSpec(result))
}

fn parse_format_string(spec: &str) -> Result<(Vec<FormatSpec>, usize)> {
    use self::FormatSpec::*;

    let mut vec   = Vec::new();
    let mut count = 0;
    let mut buf   = String::new();

    let mut chars = spec.chars();

    while let Some(c) = chars.next() {
        let mut align = |buf: &mut String, format_spec: FormatSpec| {
            if !buf.is_empty() {
                vec.push(Literal(std::mem::replace(buf, String::new())));
            }
            vec.push(format_spec);
            count += 1;
        };

        match c {
            '{' => {
                let col_spec = get_column_spec(&mut chars)?;

                match col_spec.as_str() {
                    ":<" => align(&mut buf, Left),
                    ":>" => align(&mut buf, Right),
                    _    => return Err(Error::BadColumnSpec(col_spec)),
                }

            }

            '}' => {
                if chars.next() == Some('}') {
                    buf.push('}');
                } else {
                    return Err(Error::UnexpectedRightBrace);
                }
            }

            _ => buf.push(c),
        }
    }

    if !buf.is_empty() {
        vec.push(Literal(buf));
    }

    Ok((vec, count))
}

#[derive(Clone, Debug)]
enum InternalRow {
    Cells(Vec<WidthString>),
    Heading(String),
}

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
    format:        Vec<FormatSpec>,
    rows:          Vec<InternalRow>,
    column_widths: Vec<usize>,
}

impl Table {
    pub fn new(format_string: &str) -> Self {
        Self::new_safe(format_string).unwrap_or_else(|e|
            panic!("tabular::Table::new: {}", e))
    }

    pub fn new_safe(format_string: &str) -> Result<Self> {
        let (format, n_columns) = parse_format_string(format_string)?;
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
            *width = std::cmp::max(*width, s.width());
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
    //   - format_string_to_string
    //   - row.clone()
    // It doesn't need to do either.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Table::new({:?})", format_string_to_string(&self.format))?;

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
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use self::FormatSpec::*;

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

#[derive(Clone)]
struct WidthString {
    string: String,
    width: usize,
}

impl WidthString {
    fn new<T: Display>(thing: T) -> Self {
        let string = thing.to_string();
        #[cfg(feature = "unicode-width")]
        let width  = unicode_width::UnicodeWidthStr::width(string.as_str());
        #[cfg(not(feature = "unicode-width"))]
        let width  = string.len();
        WidthString { string, width }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn as_str(&self) -> &str {
        &self.string
    }
}

impl Debug for WidthString {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.string)
    }
}

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
        let row = Row::from_cells(vec!["a", "b", "c"]);
        eprintln!("{:?}", row);

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
