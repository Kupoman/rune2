use std::collections::VecDeque;
use super::super::source_span::SourceSpan;
use super::super::token::Token;
use super::{ParseResult, ParseError, Parseable};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct LitInteger<'a> {
    pub text: &'a str,
    pub source: SourceSpan<'a>,
}

//=============================
impl<'a> Parseable<'a> for LitInteger<'a> {
    fn parse(tokens: &'a [Token<'a>]) -> ParseResult<'a, Self> {    
        // Attempt to parse an integer literal
        if let Some(&Token::LIT_Int(s)) = tokens.get(0) {
            return Ok((
                LitInteger {
                    text: s.span,
                    source: s,
                },
                &tokens[1..],
            ));
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
}