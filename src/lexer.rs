#![allow(dead_code)]

use regex::Regex;
use token::{Token, TokenType};

/// Lexes a string slice into an vector of tokens
pub fn lex_str<'a>(text: &'a str) -> Vec<Token<'a>> {
    Lexer::new(text).lex()
}


////////////////////////////////////////////////
struct Lexer<'a> {
    text: &'a str,
    current_line: u32,
    current_column: u32,
    current_byte_offset: usize,
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(text: &'a str) -> Lexer<'a> {
        Lexer {
            text: text,
            current_line: 0,
            current_column: 0,
            current_byte_offset: 0,
            tokens: vec!(),
        }
    }
    
    // Lexing consumes the lexer
    fn lex(mut self) -> Vec<Token<'a>> {
        //==================================
        // Build our regexes
        // TODO: can we do this outside of the lex function, perhaps
        // at program initialization?
        
        // Non-newline whitespace
        let re_whitespace = Regex::new(r"[ \t]+").unwrap();

        // A single newline
        let re_newline = Regex::new(r"(?:\r\n|\r|\n)").unwrap();
        
        // A newline followed by any number of newlines and whitespace, ending
        // in a newline as well.
        let re_newlines = Regex::new(r"[\r\n](?:[\r\n \t]*[\r\n])*").unwrap();

        // Literals
        //let re_int = Regex::new(r"[0-9]+").unwrap();
        //let re_real = Regex::new(r"[0-9]+\.[0-9]+").unwrap();

        // Other
        let re_ident = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]+").unwrap();
        
        
        //==================================
        //  Do the lexing
        loop {
            let mut bytes_consumed;
            
            // End of file
            if self.text.len() == 0 {
                self.tokens.push(Token {
                    token_type: TokenType::EOF,
                    text: self.text,
                    line: self.current_line,
                    column: self.current_column,
                    byte_offset: self.current_byte_offset,
                });
                break;
            }
            
            // Newline
            else if let Some((0, n)) = re_newlines.find(self.text) {
                bytes_consumed = n;
                self.tokens.push(Token {
                    token_type: TokenType::NewLine,
                    text: &self.text[0..n],
                    line: self.current_line,
                    column: self.current_column,
                    byte_offset: self.current_byte_offset,
                });
                
                // Handle state updates specially
                self.current_line += re_newline.find_iter(&self.text[0..n]).count() as u32;
                self.current_column = 0;
                self.current_byte_offset += bytes_consumed;
                self.text = &self.text[bytes_consumed..];
                continue;
            }
            
            // White space
            else if let Some((0, n)) = re_whitespace.find(self.text) {
                bytes_consumed = n;
            }
            
            // Identifier or keyword
            else if let Some((0, n)) = re_ident.find(self.text) {
                bytes_consumed = n;
                
                let tt = match &self.text[0..n] {
                    "namespace" => TokenType::KEY_Namespace,
                    "pub" => TokenType::KEY_Pub,
                    "unsafe" => TokenType::KEY_Unsafe,
                    "const" => TokenType::KEY_Const,
                    "val" => TokenType::KEY_Val,
                    "var" => TokenType::KEY_Var,
                    "mut" => TokenType::KEY_Mut,
                    "ref" => TokenType::KEY_Ref,
                    "fn" => TokenType::KEY_Fn,
                    "struct" => TokenType::KEY_Struct,
                    "enum" => TokenType::KEY_Enum,
                    "union" => TokenType::KEY_Union,
                    "trait" => TokenType::KEY_Trait,
                    "is" => TokenType::KEY_Is,
                    "if" => TokenType::KEY_If,
                    "else" => TokenType::KEY_Else,
                    "loop" => TokenType::KEY_Loop,
                    "while" => TokenType::KEY_While,
                    "until" => TokenType::KEY_Until,
                    "for" => TokenType::KEY_For,
                    "in" => TokenType::KEY_In,
                    "break" => TokenType::KEY_Break,
                    "continue" => TokenType::KEY_Continue,
                    "return" => TokenType::KEY_Return,
                    "as" => TokenType::KEY_As,
                    "alias" => TokenType::KEY_Alias,
                    "type" => TokenType::KEY_Type,
                    
                    _ => TokenType::Identifier,
                };
                
                self.tokens.push(Token {
                    token_type: tt,
                    text: &self.text[0..n],
                    line: self.current_line,
                    column: self.current_column,
                    byte_offset: self.current_byte_offset,
                });
            }
            
            // 'var' keyword
            else if let Some((0, n)) = re_ident.find(self.text) {
                bytes_consumed = n;
                self.tokens.push(Token {
                    token_type: TokenType::Identifier,
                    text: &self.text[0..n],
                    line: self.current_line,
                    column: self.current_column,
                    byte_offset: self.current_byte_offset,
                });
            }
            
            // Unknown input text
            else {
                panic!("Error: unknown text!");
            }
            
            // Update state
            self.current_column += bytes_consumed as u32; // TODO: actually base this on grapheme count
            self.current_byte_offset += bytes_consumed;
            self.text = &self.text[bytes_consumed..];
        }
        
        return self.tokens;
    }
    
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;
    
    #[test]
    fn t1() {
        let tokens = lex_str("var hello");
        
        assert_eq!(tokens[0].token_type, TokenType::KEY_Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
    }
}