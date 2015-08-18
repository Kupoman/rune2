/// A struct that holds all the data relevant to a span of source code.
/// This is included in nodes throughout the lexing and parsing process
/// for a variety of purposes, including error messages.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct SourceSpan<'a> {
    pub span: &'a str,
    pub full_source_text: &'a str,
    pub byte_offset: usize,
    pub line: u32,
    pub column: u32,
}

impl<'a> SourceSpan<'a> {
    pub fn text(&'a self) -> &'a str {
        self.span
    }
}