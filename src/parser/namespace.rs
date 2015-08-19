use super::declaration::Declaration;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Namespace<'a> {
    decls: &'a [Declaration<'a>],
}