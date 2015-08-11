#![allow(dead_code)]

use std::collections::HashMap;
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
        let re_newline = Regex::new(r"(\r\n|\r|\n)").unwrap();

        // A comment
        let re_comment = Regex::new(r"#[^\r\n]*").unwrap();
        
        // A doc comment
        let re_doc_comment = Regex::new(r"#:[^\r\n]*").unwrap();
        
        // Literals
        let re_int = Regex::new(r"[0-9]+").unwrap();
        let re_real = Regex::new(r"[0-9]+\.[0-9]+").unwrap();

        // Identifiers
        let re_ident_or_keyword = Regex::new(r"[a-zA-Z][a-zA-Z0-9_]+").unwrap();
        let re_ident_generic = Regex::new(r"_[a-zA-Z][a-zA-Z0-9_]+").unwrap();
        
        // Operators
        let re_operator = Regex::new(r"[-+/*%|&!~]+").unwrap();
        
        //===================================
        // Set up map for single-character tokens
        let mut single_byte_tokens = HashMap::new();
        single_byte_tokens.insert("(", TokenType::LParen);
        single_byte_tokens.insert(")", TokenType::RParen);
        single_byte_tokens.insert("[", TokenType::LSquare);
        single_byte_tokens.insert("]", TokenType::RSquare);
        single_byte_tokens.insert("{", TokenType::LCurly);
        single_byte_tokens.insert("}", TokenType::RCurly);
        single_byte_tokens.insert(",", TokenType::Comma);
        single_byte_tokens.insert(":", TokenType::Colon);
        single_byte_tokens.insert("@", TokenType::At);
        single_byte_tokens.insert(".", TokenType::Period);
        single_byte_tokens.insert("`", TokenType::BackTick);
        single_byte_tokens.insert("$", TokenType::Dollar);
        
        
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
            else if let Some((0, n)) = re_newline.find(self.text) {
                bytes_consumed = n;
                self.tokens.push(Token {
                    token_type: TokenType::NewLine,
                    text: &self.text[0..n],
                    line: self.current_line,
                    column: self.current_column,
                    byte_offset: self.current_byte_offset,
                });
                
                // Handle state updates specially
                self.current_line += 1;
                self.current_column = 0;
                self.current_byte_offset += bytes_consumed;
                self.text = &self.text[bytes_consumed..];
                continue;
            }
            
            // Doc comment
            else if let Some((0, n)) = re_doc_comment.find(self.text) {
                bytes_consumed = n;
                self.tokens.push(Token {
                    token_type: TokenType::DocComment,
                    text: &self.text[0..1],
                    line: self.current_line,
                    column: self.current_column,
                    byte_offset: self.current_byte_offset,
                });
            }
            
            // Comment
            else if let Some((0, n)) = re_comment.find(self.text) {
                bytes_consumed = n;
            }
            
            // White space
            else if let Some((0, n)) = re_whitespace.find(self.text) {
                bytes_consumed = n;
            }
            
            // Punctuation
            else if let Some(tt) = single_byte_tokens.get(&self.text[0..1]) {
                bytes_consumed = 1;
                self.tokens.push(Token {
                    token_type: *tt,
                    text: &self.text[0..1],
                    line: self.current_line,
                    column: self.current_column,
                    byte_offset: self.current_byte_offset,
                });
            }
            
            // Operators
            else if let Some((0, n)) = re_operator.find(self.text) {
                bytes_consumed = n;
                self.tokens.push(Token {
                    token_type: TokenType::Operator,
                    text: &self.text[0..1],
                    line: self.current_line,
                    column: self.current_column,
                    byte_offset: self.current_byte_offset,
                });
            }
            
            // Real number literal
            else if let Some((0, n)) = re_real.find(self.text) {
                bytes_consumed = n;
                self.tokens.push(Token {
                    token_type: TokenType::LIT_Real,
                    text: &self.text[0..1],
                    line: self.current_line,
                    column: self.current_column,
                    byte_offset: self.current_byte_offset,
                });
            }
            
            // Integer literal
            else if let Some((0, n)) = re_int.find(self.text) {
                bytes_consumed = n;
                self.tokens.push(Token {
                    token_type: TokenType::LIT_Int,
                    text: &self.text[0..1],
                    line: self.current_line,
                    column: self.current_column,
                    byte_offset: self.current_byte_offset,
                });
            }
            
            // Identifier or keyword
            else if let Some((0, n)) = re_ident_or_keyword.find(self.text) {
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
            
            // Identifier of a generic parameter
            else if let Some((0, n)) = re_ident_generic.find(self.text) {
                bytes_consumed = n;
                
                self.tokens.push(Token {
                    token_type: TokenType::IdentifierGeneric,
                    text: &self.text[0..n],
                    line: self.current_line,
                    column: self.current_column,
                    byte_offset: self.current_byte_offset,
                });
            }
            
            // Unknown input text
            else {
                println!("{:?}", self.text);
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
    fn idents_and_keywords() {
        let tokens = lex_str("var hello");
        
        assert_eq!(tokens[0].token_type, TokenType::KEY_Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::EOF);
    }
    
    #[test]
    fn newlines() {
        let tokens = lex_str("var\n \n   \n hello");
        
        assert_eq!(tokens[0].token_type, TokenType::KEY_Var);
        assert_eq!(tokens[1].token_type, TokenType::NewLine);
        assert_eq!(tokens[2].token_type, TokenType::NewLine);
        assert_eq!(tokens[3].token_type, TokenType::NewLine);
        assert_eq!(tokens[4].token_type, TokenType::Identifier);
        assert_eq!(tokens[5].token_type, TokenType::EOF);
    }
    
    #[test]
    fn punctuation() {
        let tokens = lex_str("{}()[]@.,:`$");
        
        assert_eq!(tokens[0].token_type, TokenType::LCurly);
        assert_eq!(tokens[1].token_type, TokenType::RCurly);
        assert_eq!(tokens[2].token_type, TokenType::LParen);
        assert_eq!(tokens[3].token_type, TokenType::RParen);
        assert_eq!(tokens[4].token_type, TokenType::LSquare);
        assert_eq!(tokens[5].token_type, TokenType::RSquare);
        assert_eq!(tokens[6].token_type, TokenType::At);
        assert_eq!(tokens[7].token_type, TokenType::Period);
        assert_eq!(tokens[8].token_type, TokenType::Comma);
        assert_eq!(tokens[9].token_type, TokenType::Colon);
        assert_eq!(tokens[10].token_type, TokenType::BackTick);
        assert_eq!(tokens[11].token_type, TokenType::Dollar);
        assert_eq!(tokens[12].token_type, TokenType::EOF);
    }
    
    #[test]
    fn operator() {
        let tokens = lex_str("- + / * % | & ! ~ ++-*&|%");
        
        assert_eq!(tokens[0].token_type, TokenType::Operator);
        assert_eq!(tokens[1].token_type, TokenType::Operator);
        assert_eq!(tokens[2].token_type, TokenType::Operator);
        assert_eq!(tokens[3].token_type, TokenType::Operator);
        assert_eq!(tokens[4].token_type, TokenType::Operator);
        assert_eq!(tokens[5].token_type, TokenType::Operator);
        assert_eq!(tokens[6].token_type, TokenType::Operator);
        assert_eq!(tokens[7].token_type, TokenType::Operator);
        assert_eq!(tokens[8].token_type, TokenType::Operator);
        assert_eq!(tokens[9].token_type, TokenType::Operator);
        assert_eq!(tokens[10].token_type, TokenType::EOF);
    }
    
    #[test]
    fn ints_and_reals() {
        let tokens = lex_str("123 12.3");
        
        assert_eq!(tokens[0].token_type, TokenType::LIT_Int);
        assert_eq!(tokens[1].token_type, TokenType::LIT_Real);
        assert_eq!(tokens[2].token_type, TokenType::EOF);
    }
    
    #[test]
    fn comments() {
        let tokens = lex_str("var hello# How's it going?\n");
        
        assert_eq!(tokens[0].token_type, TokenType::KEY_Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::NewLine);
        assert_eq!(tokens[3].token_type, TokenType::EOF);
    }
    
    #[test]
    fn doc_comments() {
        let tokens = lex_str("var hello#: How's it going?\n");
        
        assert_eq!(tokens[0].token_type, TokenType::KEY_Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::DocComment);
        assert_eq!(tokens[3].token_type, TokenType::NewLine);
        assert_eq!(tokens[4].token_type, TokenType::EOF);
    }
}