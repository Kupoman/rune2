mod declaration;
mod namespace;
mod lit_integer;


use token::Token;
use self::namespace::Namespace;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ParseTree<'a> {
    Empty,
    Root(Namespace<'a>)
}