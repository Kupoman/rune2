#![allow(dead_code)]
#![allow(non_camel_case_types)]

use source_span::SourceSpan;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub source: SourceSpan<'a>,
}


#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TokenType {
    // Catch-all
    Unknown,
    
    // Catch-all for valid but as-of-yet unused symbols
    Reserved,

    // User-defined symbols
    Identifier,
    IdentifierGeneric,
    Operator,

    // Literals
    LIT_Int,
    LIT_Real,
    LIT_String,
    LIT_RawString,

    // Documentation string
    DocComment,

    // Punctuation
    NewLine,
    LParen,
    RParen,
    LSquare,
    RSquare,
    LCurly,
    RCurly,
    Comma,
    Colon,
    At,
    Period,
    BackTick,
    Dollar,

    // Keywords
    KEY_Namespace,
    KEY_Pub,
    KEY_Unsafe,

    KEY_Const,
    KEY_Val,
    KEY_Var,

    KEY_Mut,
    KEY_Ref,

    KEY_Fn,

    KEY_Struct,
    KEY_Enum,
    KEY_Union,

    KEY_Trait,
    KEY_Is,

    KEY_If,
    KEY_Else,
    KEY_Loop,
    KEY_While,
    KEY_Until,
    KEY_For,
    KEY_In,
    KEY_Break,
    KEY_Continue,
    KEY_Return,

    KEY_As,

    KEY_Alias,
    KEY_Type,

    // EOF
    EOF,
}
