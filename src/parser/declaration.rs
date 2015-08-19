use super::namespace::Namespace;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Declaration<'a> {
    Namespace(Namespace<'a>),
}