use std::collections::VecDeque;
use super::super::source_span::SourceSpan;
use super::super::token::Token;
use super::{ParseResult, ParseError, Parseable};

use super::lit_integer::LitInteger;
use super::lit_real::LitReal;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Expression<'a> {
    LitInteger(LitInteger<'a>),
    LitReal(LitReal<'a>),
}

//=============================
impl<'a> Parseable<'a> for Expression<'a> {
    fn parse(tokens: &'a [Token<'a>]) -> ParseResult<'a, Self> {    
        // Integer literal
        if let Ok((lit_int, rem_tokens)) = LitInteger::parse(tokens) {
            return Ok((
                Expression::LitInteger(lit_int),
                rem_tokens,
            ));
        }
        // Real number literal
        else if let Ok((lit_real, rem_tokens)) = LitReal::parse(tokens) {
            return Ok((
                Expression::LitReal(lit_real),
                rem_tokens,
            ));
        }
        // Error, no successful expression parse
        else {
            let ss = if let Some(token) = tokens.get(0) { token.source_span() } else { None };
            return Err(ParseError {
                message: "Expected expression.".to_string(),
                source: ss,
            });
        }
    }
}