use std::collections::VecDeque;
use super::super::source_span::SourceSpan;
use super::super::token::Token;
use super::{ParseResult, ParseError};

use super::lit_integer{LitInteger, parse_lit_integer};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct InfixOpCall<'a> {
    pub text: &'a str,
    pub source: SourceSpan<'a>,
}


//=============================
pub fn parse_infix_op_call<'a>(tokens: &[Token<'a>]) -> ParseResult<'a, LitReal<'a>> {
    // Attempt to parse an real literal
    if let Some(&Token::LIT_Real(s)) = tokens.get(0) {
        tokens.pop_front();
        return Ok((
            LitReal {
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
            message: "Expected real number literal.".to_string(),
            source: ss,
        });
    }
}


