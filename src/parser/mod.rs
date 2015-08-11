mod namespace;

use token::{Token, TokenType};
use self::namespace::Namespace;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ParseTree<'a> {
    Empty,
    Root(Namespace<'a>)
}