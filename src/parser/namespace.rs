#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Namespace<'a> {
    Foo,
    Bar(&'a str),
}