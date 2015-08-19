use std::collections::VecDeque;
use super::super::source_span::SourceSpan;
use super::super::token::Token;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct LitInteger<'a> {
    pub text: &'a str,
    pub source: SourceSpan<'a>,
}

pub fn parse_lit_integer<'a>(tokens: &mut VecDeque<Token<'a>>) -> Option<LitInteger<'a>> {
    // Attempt to parse an integer literal
    if let Some(&Token::LIT_Int(s)) = tokens.get(0) {
        tokens.pop_front();
        return Some(LitInteger {
            text: s.span,
            source: s,
        });
    }
    else {
        return None;
    }
}


