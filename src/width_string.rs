use std::fmt::{Debug, Formatter};
use unicode_width::UnicodeWidthStr;

#[derive(Clone, PartialOrd, PartialEq, Eq)]
pub struct WidthString {
    string: String,
    width: usize,
}

impl<S> From<S> for WidthString where S: ToString {
    fn from(s: S) -> Self {
        let string = s.to_string();
        #[cfg(feature = "unicode-width")]
        let width  = ::unicode_width::UnicodeWidthStr::width(string.as_str());
        #[cfg(not(feature = "unicode-width"))]
        let width  = string.chars().count();
        WidthString { string, width }
    }
}

impl WidthString {
    pub fn new<T>(thing: T, width: usize) -> Self where T: Into<String> {
        WidthString { string: thing.into(), width }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn as_str(&self) -> &str {
        &self.string
    }
}

impl Debug for WidthString {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self.string)
    }
}

impl Default for WidthString {
    fn default() -> Self {
        WidthString {
            string: String::new(),
            width: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Custom {
        field: i32,
    }

    impl From<Custom> for WidthString {
        fn from(_: Custom) -> Self {
            Self { string: "foo".to_string(), width: 20 }
        }
    }

    #[test]
    fn ascii_string() {
        let expected = WidthString { width: 2, string: "ab".to_string() };
        let result: WidthString = "ab".to_string().into();
        assert_eq!(result, expected);
    }

    #[test]
    fn string_slice() {
        let expected = WidthString { width: 2, string: "ab".to_string() };
        let result: WidthString = "ab".into();
        assert_eq!(result, expected);
    }

    #[test]
    fn i32() {
        let expected = WidthString { width: 2, string: "12".to_string() };
        let result: WidthString = 12.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn f32() {
        let expected = WidthString { width: 2, string: "12".to_string() };
        let result: WidthString = 12f32.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn custom() {
        let expected = WidthString { width: 20, string: "foo".to_string() };
        let result: WidthString = Custom { field: 12 } .into();
        assert_eq!(result, expected);
    }
}
