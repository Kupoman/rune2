use super::super::source_span::SourceSpan;
use super::super::token::Token;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct LitInteger<'a> {
    pub text: &'a str,
    pub source: SourceSpan<'a>,
}

//pub fn parse_lit_integer<'a>(tokens: &mut Vec<Token<'a>>) -> Option<LitInteger<'a>> {
//    match 
//    LitInteger {
//        text: 
//    }
//}


