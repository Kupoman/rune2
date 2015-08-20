use std::collections::VecDeque;
use super::super::source_span::SourceSpan;
use super::super::token::Token;
use super::{ParseResult, ParseError};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct LitReal<'a> {
    pub text: &'a str,
    pub source: SourceSpan<'a>,
}

//=============================
pub fn parse_lit_real<'a>(tokens: &'a [Token<'a>]) -> ParseResult<'a, LitReal<'a>> {
    // Attempt to parse an real literal
    if let Some(&Token::LIT_Real(s)) = tokens.get(0) {
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


