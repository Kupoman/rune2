mod declaration;
mod expression;
mod namespace;
mod lit_integer;
mod lit_real;

use std::fmt::{Display, Formatter, Error};
use source_span::SourceSpan;
use token::Token;
use self::namespace::Namespace;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ParseTree<'a> {
    Empty,
    Root(Namespace<'a>)
}


//=========================
/// A trait for nodes in a parse tree that can be parsed from a token slice.
pub trait Parseable<'a> {
    fn parse(tokens: &'a [Token<'a>]) -> ParseResult<'a, Self>;
}


//=====================================
// A parse error.
#[derive(Clone, Debug)]
pub struct ParseError<'a> {
    pub message: String,
    pub source: Option<SourceSpan<'a>>,
}

// Make it easy to print parse errors in a uniform way.
impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if let Some(ss) = self.source {
            f.write_str(&format!("Parse Error [{}:{}]: {}", ss.line+1, ss.column, &self.message[..])[..])
        }
        else {
            f.write_str(&format!("Parse Error: {}", &self.message[..])[..])
        }
    }
}

// Alias for a result using a ParseError
pub type ParseResult<'a, T> = Result<(T, &'a[Token<'a>]), ParseError<'a>>;