use std::collections::VecDeque;
use super::super::source_span::SourceSpan;
use super::super::token::Token;
use super::{ParseResult, ParseError};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct LitInteger<'a> {
    pub text: &'a str,
    pub source: SourceSpan<'a>,
}

//=============================
pub fn parse_lit_integer<'a>(tokens: &mut VecDeque<Token<'a>>) -> ParseResult<'a, LitInteger<'a>> {
    // Attempt to parse an integer literal
    if let Some(&Token::LIT_Int(s)) = tokens.get(0) {
        tokens.pop_front();
        return Ok(LitInteger {
            text: s.span,
            source: s,
        });
    }
    // Return error if failed
    else {
        let ss = if let Some(token) = tokens.get(0) { token.source_span() } else { None };
        return Err(ParseError {
            message: "Expected integer literal.".to_string(),
            source: ss,
        });
    }
}


