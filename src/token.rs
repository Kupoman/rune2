#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::fmt::{Display, Formatter, Error};
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


impl<'a> Token<'a> {
    pub fn source_span(&self) -> Option<SourceSpan<'a>> {
        match *self {
            // Catch-all
            Token::Unknown(ss) => Some(ss),
            
            // Catch-all for valid but as-of-yet unused symbols
            Token::Reserved(ss) => Some(ss),

            // User-defined symbols
            Token::Identifier(ss) => Some(ss),
            Token::IdentifierGeneric(ss) => Some(ss),
            Token::Operator(ss) => Some(ss),

            // Literals
            Token::LIT_Int(ss) => Some(ss),
            Token::LIT_Real(ss) => Some(ss),
            Token::LIT_String(ss) => Some(ss),
            Token::LIT_RawString(ss) => Some(ss),

            // Documentation string
            Token::DocComment(ss) => Some(ss),

            // Punctuation
            Token::NewLine(ss) => Some(ss),
            Token::LParen(ss) => Some(ss),
            Token::RParen(ss) => Some(ss),
            Token::LSquare(ss) => Some(ss),
            Token::RSquare(ss) => Some(ss),
            Token::LCurly(ss) => Some(ss),
            Token::RCurly(ss) => Some(ss),
            Token::Comma(ss) => Some(ss),
            Token::Colon(ss) => Some(ss),
            Token::At(ss) => Some(ss),
            Token::Period(ss) => Some(ss),
            Token::BackTick(ss) => Some(ss),
            Token::Dollar(ss) => Some(ss),

            // Keywords
            Token::KEY_Namespace(ss) => Some(ss),
            Token::KEY_Pub(ss) => Some(ss),
            Token::KEY_Unsafe(ss) => Some(ss),

            Token::KEY_Const(ss) => Some(ss),
            Token::KEY_Val(ss) => Some(ss),
            Token::KEY_Var(ss) => Some(ss),

            Token::KEY_Mut(ss) => Some(ss),
            Token::KEY_Ref(ss) => Some(ss),

            Token::KEY_Fn(ss) => Some(ss),

            Token::KEY_Struct(ss) => Some(ss),
            Token::KEY_Enum(ss) => Some(ss),
            Token::KEY_Union(ss) => Some(ss),

            Token::KEY_Trait(ss) => Some(ss),
            Token::KEY_Is(ss) => Some(ss),

            Token::KEY_If(ss) => Some(ss),
            Token::KEY_Else(ss) => Some(ss),
            Token::KEY_Loop(ss) => Some(ss),
            Token::KEY_While(ss) => Some(ss),
            Token::KEY_Until(ss) => Some(ss),
            Token::KEY_For(ss) => Some(ss),
            Token::KEY_In(ss) => Some(ss),
            Token::KEY_Break(ss) => Some(ss),
            Token::KEY_Continue(ss) => Some(ss),
            Token::KEY_Return(ss) => Some(ss),

            Token::KEY_As(ss) => Some(ss),

            Token::KEY_Alias(ss) => Some(ss),
            Token::KEY_Type(ss) => Some(ss),

            // EOF
            Token::EOF => None,
        }
    }
}



// Prettier printing of tokens
impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let debug_string = format!("{:?}", *self);
        let type_name = &debug_string[..].split("(").next().unwrap();
        
        if let Some(ss) = self.source_span() {
            f.write_str(&format!("{}: [{}:{}]  \t{}", type_name, ss.line, ss.column, ss.span)[..])
        }
        else {
            f.write_str(&format!("{}", type_name)[..])    
        }
    }
}

