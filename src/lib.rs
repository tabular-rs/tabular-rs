#[cfg(feature = "unicode-width")]
extern crate unicode_width;

use std::fmt::Display;

pub struct Row(Vec<WidthString>);

#[derive(Debug, Clone)]
pub enum Error {
    UnclosedColumnSpec(String),
    BadFormatSpec(String),
    UnexpectedRightBrace,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnclosedColumnSpec(_) => "unclosed column specifier",
            Error::BadFormatSpec(_) => "bad format specifier",
            Error::UnexpectedRightBrace => "unexpected single ‘}’ character",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::UnclosedColumnSpec(ref spec) =>
                write!(f, "unclosed column specifier: ‘{{{}’", spec),
            Error::BadFormatSpec(ref spec) =>
                write!(f, "bad format specifier: ‘{{{}}}’", spec),
            Error::UnexpectedRightBrace =>
                f.write_str("unexpected single ‘}’ character"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl Row {
    pub fn new() -> Self {
        Row(Vec::new())
    }

    pub fn add_cell<S: Display>(mut self, value: S) -> Self {
        self.0.push(WidthString::new(value));
        self
    }
}

enum FormatSpec {
    Left,
    Right,
    Literal(String),
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
                    _    => return Err(Error::BadFormatSpec(col_spec)),
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

enum InternalRow {
    Cells(Vec<WidthString>),
    Heading(String),
}

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

    pub fn add_heading(&mut self, heading: String) -> &mut Self {
        self.rows.push(InternalRow::Heading(heading));
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
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment() {
        let mut table = Table::new("{:>}  ({:<}) {:<}");
        table
            .add_row(Row::new().add_cell(1).add_cell("I").add_cell("one"))
            .add_row(Row::new().add_cell(5).add_cell("V").add_cell("five"))
            .add_row(Row::new().add_cell(10).add_cell("X").add_cell("ten"))
            .add_row(Row::new().add_cell(50).add_cell("L").add_cell("fifty"))
            .add_row(Row::new().add_cell(100).add_cell("C").add_cell("one-hundred"));
        assert_eq!( format!("\n{}", table),
                    r#"
  1  (I) one
  5  (V) five
 10  (X) ten
 50  (L) fifty
100  (C) one-hundred
"# );
    }
}
