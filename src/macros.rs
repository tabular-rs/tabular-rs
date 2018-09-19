#[macro_export]

/// A macro for building a [`Row`].
///
/// `$row(A, B, C)` is equivalent to
/// `Row::new().with_cell(A).with_cell(B).with_cell(B)`.
///
/// # Examples
///
/// ```
/// #[macro_use(row)]
/// extern crate tabular;
///
/// # fn main() {
/// let table = tabular::Table::new("{:>}  {:<}  {:<}")
///     .with_row(row!(34, "hello", true))
///     .with_row(row!(567, "goodbye", false));
///
/// assert_eq!( format!("\n{}", table),
///             r#"
///  34  hello    true
/// 567  goodbye  false
/// "# );
/// # }
/// ```
///
/// [`Row`]: struct.Row.html
macro_rules! row {
    ( $( $cell:expr ),* ) => {
        {
            let mut result = $crate::Row::new();
            $(
                result.add_cell($cell);
            )*
            result
        }
    };

    ( $( $cell:expr, )* ) => {
        row!( $( $cell ),* )
    };
}