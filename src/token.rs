#![allow(dead_code)]
#![allow(non_camel_case_types)]

use source_span::SourceSpan;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Token<'a> {
    // Catch-all
    Unknown(SourceSpan<'a>),
    
    // Catch-all for valid but as-of-yet unused symbols
    Reserved(SourceSpan<'a>),

    // User-defined symbols
    Identifier(SourceSpan<'a>),
    IdentifierGeneric(SourceSpan<'a>),
    Operator(SourceSpan<'a>),

    // Literals
    LIT_Int(SourceSpan<'a>),
    LIT_Real(SourceSpan<'a>),
    LIT_String(SourceSpan<'a>),
    LIT_RawString(SourceSpan<'a>),

    // Documentation string
    DocComment(SourceSpan<'a>),

    // Punctuation
    NewLine(SourceSpan<'a>),
    LParen(SourceSpan<'a>),
    RParen(SourceSpan<'a>),
    LSquare(SourceSpan<'a>),
    RSquare(SourceSpan<'a>),
    LCurly(SourceSpan<'a>),
    RCurly(SourceSpan<'a>),
    Comma(SourceSpan<'a>),
    Colon(SourceSpan<'a>),
    At(SourceSpan<'a>),
    Period(SourceSpan<'a>),
    BackTick(SourceSpan<'a>),
    Dollar(SourceSpan<'a>),

    // Keywords
    KEY_Namespace(SourceSpan<'a>),
    KEY_Pub(SourceSpan<'a>),
    KEY_Unsafe(SourceSpan<'a>),

    KEY_Const(SourceSpan<'a>),
    KEY_Val(SourceSpan<'a>),
    KEY_Var(SourceSpan<'a>),

    KEY_Mut(SourceSpan<'a>),
    KEY_Ref(SourceSpan<'a>),

    KEY_Fn(SourceSpan<'a>),

    KEY_Struct(SourceSpan<'a>),
    KEY_Enum(SourceSpan<'a>),
    KEY_Union(SourceSpan<'a>),

    KEY_Trait(SourceSpan<'a>),
    KEY_Is(SourceSpan<'a>),

    KEY_If(SourceSpan<'a>),
    KEY_Else(SourceSpan<'a>),
    KEY_Loop(SourceSpan<'a>),
    KEY_While(SourceSpan<'a>),
    KEY_Until(SourceSpan<'a>),
    KEY_For(SourceSpan<'a>),
    KEY_In(SourceSpan<'a>),
    KEY_Break(SourceSpan<'a>),
    KEY_Continue(SourceSpan<'a>),
    KEY_Return(SourceSpan<'a>),

    KEY_As(SourceSpan<'a>),

    KEY_Alias(SourceSpan<'a>),
    KEY_Type(SourceSpan<'a>),

    // EOF
    EOF,
}
