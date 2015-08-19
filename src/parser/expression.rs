use std::collections::VecDeque;
use super::super::source_span::SourceSpan;
use super::super::token::Token;
use super::{ParseResult, ParseError};

use super::lit_integer::{LitInteger, parse_lit_integer};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Expression<'a> {
    LitInteger(LitInteger<'a>),
}

//=============================
pub fn parse_expression<'a>(tokens: &mut VecDeque<Token<'a>>) -> ParseResult<'a, Expression<'a>> {
    match parse_lit_integer(tokens) {
        Ok(lit_int) => Ok(Expression::LitInteger(lit_int)),
        Err(e) => Err(e),
    }
}