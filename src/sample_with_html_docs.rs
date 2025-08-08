/// This function does something interesting with <code>&lt;T&gt;</code> types.
/// 
/// It converts from one type to another using the implementation of
/// <code>[From]&lt;T&gt; for U</code> that is available.
/// 
/// # Example
/// 
/// ```rust
/// let value: String = some_value.into();
/// ```
/// 
/// See also <em>emphasis</em> and <strong>bold</strong> text.
pub fn sample_function<T, U>(input: T) -> U
where
    U: From<T>,
{
    input.into()
}

/// A sample struct with <code>&lt;Generic&gt;</code> parameter.
/// 
/// This struct shows how <strong>HTML entities</strong> are converted
/// to proper markdown in the generated documentation.
pub struct SampleStruct<T> {
    /// Field with <code>&lt;T&gt;</code> type and &amp; ampersand
    pub field: T,
}