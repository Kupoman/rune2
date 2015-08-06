use std::str::CharIndices;
use token::{Token, TokenType};

pub struct Lexer<'a> {
    chars: CharIndices<'a>,
    current_line: u32,
    current_column: u32,
    current_byte_offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Lexer<'a> {
        Lexer {
            chars: text.char_indices(),
            current_line: 0,
            current_column: 0,
            current_byte_offset: 0,
        }
    }
    
    pub fn lex(&'a mut self) -> Vec<Token<'a>> {
        vec!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use token::{Token, TokenType};
    
    #[test]
    fn t1() {
        let mut l = Lexer::new("var hello");
        let t = l.lex();
        
        assert!(if let TokenType::KEY_Var = t[0].token_type {true} else {false});
        assert!(if let TokenType::Identifier = t[1].token_type {true} else {false});
    }
}