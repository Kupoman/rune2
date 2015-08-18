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
    pub fn new_merged(s1: &SourceSpan<'a>, s2: &SourceSpan<'a>) -> SourceSpan<'a> {
        if s1.full_source_text != s2.full_source_text {
            panic!();
        }
        
        let end_a = s1.byte_offset + s1.span.len();
        let end_b = s2.byte_offset + s2.span.len();
        let end = if end_a >= end_b {end_a} else {end_b};
        
        if s1.byte_offset <= s2.byte_offset {
            return SourceSpan {
                span: &s1.full_source_text[s1.byte_offset..end],
                full_source_text: s1.full_source_text,
                byte_offset: s1.byte_offset,
                line: s1.line,
                column: s1.column,
            };
        } else {
            return SourceSpan {
                span: &s1.full_source_text[s2.byte_offset..end],
                full_source_text: s2.full_source_text,
                byte_offset: s2.byte_offset,
                line: s2.line,
                column: s2.column,
            };
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merged() {
        let text = "Hello there, how are\n you doing?";
        let span1 = SourceSpan {
            span: &text[0..22],
            full_source_text: text,
            byte_offset: 0,
            line: 0,
            column: 0,
        };
        let span2 = SourceSpan {
            span: &text[23..32],
            full_source_text: text,
            byte_offset: 23,
            line: 1,
            column: 2,
        };
        
        let span_merged1 = SourceSpan::new_merged(&span1, &span2);
        let span_merged2 = SourceSpan::new_merged(&span2, &span1);
        
        assert_eq!(span_merged1.span, text);
        assert_eq!(span_merged1.full_source_text, text);
        assert_eq!(span_merged1.byte_offset, 0);
        assert_eq!(span_merged1.line, 0);
        assert_eq!(span_merged1.column, 0);
        
        assert_eq!(span_merged2.span, text);
        assert_eq!(span_merged2.full_source_text, text);
        assert_eq!(span_merged2.byte_offset, 0);
        assert_eq!(span_merged2.line, 0);
        assert_eq!(span_merged2.column, 0);
    }
}