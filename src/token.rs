pub struct Token<'a> {
    pub token_type: TokenType,
    pub text: &'a str,
    pub line: u32,
    pub column: u32,
    pub byte_offset: usize,
}

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
    LIT_Float,
    LIT_String,
    LIT_RawString,

    // Documentation string
    DocString,

    // Punctuation
    NewLine,
    LParen,
    RParent,
    LSquare,
    RSquare,
    LCurly,
    RCurly,
    At,
    Comma,
    Period,
    Colon,
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
