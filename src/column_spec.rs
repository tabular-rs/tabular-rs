use ::error::*;

#[derive(Clone, Debug)]
pub enum ColumnSpec {
    Left,
    Right,
    Literal(String),
}

pub fn row_spec_to_string(specs: &[ColumnSpec]) -> String {
    use self::ColumnSpec::*;

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

pub fn get_column_spec(chars: &mut ::std::str::Chars) -> Result<String> {
    let mut result = String::new();

    while let Some(c) = chars.next() {
        if c == '}' {
            return Ok(result);
        }

        result.push(c);
    }

    Err(Error::UnclosedColumnSpec(result))
}

pub fn parse_row_spec(spec: &str) -> Result<(Vec<ColumnSpec>, usize)> {
    use self::ColumnSpec::*;

    let mut vec   = Vec::new();
    let mut count = 0;
    let mut buf   = String::new();

    let mut chars = spec.chars();

    while let Some(c) = chars.next() {
        let mut align = |buf: &mut String, col_spec: ColumnSpec| {
            if !buf.is_empty() {
                vec.push(Literal(::std::mem::replace(buf, String::new())));
            }
            vec.push(col_spec);
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

