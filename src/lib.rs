extern crate unicode_width;

use unicode_width::UnicodeWidthStr;
use std::fmt::Display;

pub struct Row(Vec<String>);

impl Row {
    pub fn new() -> Self {
        Row(Vec::new())
    }

    pub fn add_cell<S: Display>(mut self, value: S) -> Self {
        self.0.push(value.to_string());
        self
    }
}

enum FormatSpec {
    Left,
    Right,
    Literal(String),
}

fn parse_format_string(spec: &str) -> (Vec<FormatSpec>, usize) {
    use self::FormatSpec::*;

    let mut vec   = Vec::new();
    let mut count = 0;
    let mut buf   = String::new();

    let mut chars = spec.chars();

    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('%') => buf.push('%'),

                Some('l') => {
                    if !buf.is_empty() {
                        vec.push(Literal(buf));
                        buf = String::new();
                    }
                    vec.push(Left);
                    count += 1;
                }

                Some('r') => {
                    if !buf.is_empty() {
                        vec.push(Literal(buf));
                        buf = String::new();
                    }
                    vec.push(Right);
                    count += 1;
                }

                Some(c) => panic!("Table::new: bad format spec ‘%{}’", c),

                None    => panic!("Table::new: string ends in single %"),
            }
        } else {
            buf.push(c);
        }
    }

    if !buf.is_empty() {
        vec.push(Literal(buf));
    }

    (vec, count)
}

enum InternalRow {
    Cells(Vec<String>),
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
        let (format, n_columns) = parse_format_string(format_string);
        Table {
            n_columns,
            format,
            rows:           vec![],
            column_widths:  vec![0; n_columns]
        }
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

        for row in &self.rows {
            match row {
                InternalRow::Cells(cells) => {
                    let mut cw_iter = self.column_widths.iter().cloned();
                    let mut v_iter  = cells.iter();

                    for field in 0 .. self.format.len() {
                        let fs = &self.format[field];

                        match fs {
                            Left  => {
                                let cw = cw_iter.next().unwrap();
                                let v = match v_iter.next() {
                                    Some(v) => v.to_owned(),
                                    None    => "".to_owned(),
                                };

                                f.write_str(&v)?;

                                if field + 1 < self.format.len() {
                                    let width = cw - v.width();
                                    for _ in 0 .. width {
                                        f.write_str(" ")?;
                                    }
                                }
                            }

                            Right => {
                                let cw = cw_iter.next().unwrap();
                                let v = match v_iter.next() {
                                    Some(v) => v.to_owned(),
                                    None    => "".to_owned(),
                                };

                                let width = cw - v.width();
                                for _ in 0 .. width {
                                    f.write_str(" ")?;
                                }

                                f.write_str(&v)?;
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
