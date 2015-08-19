#![allow(dead_code)]

use std::collections::VecDeque;
use regex::Regex;
use source_span::SourceSpan;
use token::Token;

/// Lexes a string slice into an vector of tokens
pub fn lex_str<'a>(text: &'a str) -> VecDeque<Token<'a>> {
    Lexer::new(text).lex()
}

////////////////////////////////////////////////
struct Lexer<'a> {
    remaining_text: &'a str,
    full_text: &'a str,
    current_line: u32,
    current_column: u32,
    current_byte_offset: usize,
    tokens: VecDeque<Token<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(text: &'a str) -> Lexer<'a> {
        Lexer {
            remaining_text: text,
            full_text: text,
            current_line: 0,
            current_column: 0,
            current_byte_offset: 0,
            tokens: VecDeque::new(),
        }
    }
    
    
    // Lexing consumes the lexer
    fn lex(mut self) -> VecDeque<Token<'a>> {
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
        let re_raw_string_start = Regex::new("'+\"").unwrap();

        // Identifiers
        let re_ident_or_keyword = Regex::new(r"[a-zA-Z][a-zA-Z0-9_]*").unwrap();
        let re_ident_generic = Regex::new(r"_[a-zA-Z][a-zA-Z0-9_]*").unwrap();
        
        // Operators
        let re_operator = Regex::new(r"[-+/*%|&!~=<>]+").unwrap();

        // Punctuation
        let re_punctuation = Regex::new(r"[()\[\]{},:@.`$]").unwrap();
        
        //==================================
        //  Do the lexing
        loop {
            let mut bytes_consumed;
            
            // End of file
            if self.remaining_text.len() == 0 {
                self.tokens.push_back(Token::EOF);
                break;
            }
            
            // Newline
            else if let Some((0, n)) = re_newline.find(self.remaining_text) {
                bytes_consumed = n;
                self.tokens.push_back(Token::NewLine(
                    SourceSpan {
                        span: &self.remaining_text[0..n],
                        full_source_text: self.full_text,
                        byte_offset: self.current_byte_offset,
                        line: self.current_line,
                        column: self.current_column,
                    }
                ));
                
                // Handle state updates specially
                self.current_line += 1;
                self.current_column = 0;
                self.current_byte_offset += bytes_consumed;
                self.remaining_text = &self.remaining_text[bytes_consumed..];
                continue;
            }
            
            // Doc comment
            else if let Some((0, n)) = re_doc_comment.find(self.remaining_text) {
                bytes_consumed = n;
                self.tokens.push_back(Token::DocComment(
                    SourceSpan {
                        span: &self.remaining_text[0..n],
                        full_source_text: self.full_text,
                        byte_offset: self.current_byte_offset,
                        line: self.current_line,
                        column: self.current_column,
                    }
                ));
            }
            
            // Comment
            else if let Some((0, n)) = re_comment.find(self.remaining_text) {
                bytes_consumed = n;
            }
            
            // White space
            else if let Some((0, n)) = re_whitespace.find(self.remaining_text) {
                bytes_consumed = n;
            }
            
            // Punctuation
            else if let Some((0, n)) = re_punctuation.find(self.remaining_text) {
                bytes_consumed = n;
                
                let ss = SourceSpan {
                    span: &self.remaining_text[0..n],
                    full_source_text: self.full_text,
                    byte_offset: self.current_byte_offset,
                    line: self.current_line,
                    column: self.current_column,
                };
                
                self.tokens.push_back(
                    match &self.remaining_text[0..n] {
                        "(" => Token::LParen(ss),
                        ")" => Token::RParen(ss),
                        "[" => Token::LSquare(ss),
                        "]" => Token::RSquare(ss),
                        "{" => Token::LCurly(ss),
                        "}" => Token::RCurly(ss),
                        "," => Token::Comma(ss),
                        ":" => Token::Colon(ss),
                        "@" => Token::At(ss),
                        "." => Token::Period(ss),
                        "`" => Token::BackTick(ss),
                        "$" => Token::Dollar(ss),
                        _ => panic!("Unknown punctuation symbol.")
                    }
                );
            }
            
            // Operators
            else if let Some((0, n)) = re_operator.find(self.remaining_text) {
                bytes_consumed = n;
                self.tokens.push_back(Token::Operator(
                    SourceSpan {
                        span: &self.remaining_text[0..n],
                        full_source_text: self.full_text,
                        byte_offset: self.current_byte_offset,
                        line: self.current_line,
                        column: self.current_column,
                    }
                ));
            }
            
            // Real number literal
            else if let Some((0, n)) = re_real.find(self.remaining_text) {
                bytes_consumed = n;
                self.tokens.push_back(Token::LIT_Real(
                    SourceSpan {
                        span: &self.remaining_text[0..n],
                        full_source_text: self.full_text,
                        byte_offset: self.current_byte_offset,
                        line: self.current_line,
                        column: self.current_column,
                    }
                ));
            }
            
            // Integer literal
            else if let Some((0, n)) = re_int.find(self.remaining_text) {
                bytes_consumed = n;
                self.tokens.push_back(Token::LIT_Int(
                    SourceSpan {
                        span: &self.remaining_text[0..n],
                        full_source_text: self.full_text,
                        byte_offset: self.current_byte_offset,
                        line: self.current_line,
                        column: self.current_column,
                    }
                ));
            }
            
            // String literal
            else if self.remaining_text.starts_with("\"") {
                let (newline_count, trailing_txt, txt) = self.lex_string_literal();
                bytes_consumed = txt.len();
                self.tokens.push_back(Token::LIT_String(
                    SourceSpan {
                        span: &self.remaining_text[0..bytes_consumed],
                        full_source_text: self.full_text,
                        byte_offset: self.current_byte_offset,
                        line: self.current_line,
                        column: self.current_column,
                    }
                ));

                // Handle state updates specially
                self.current_line += newline_count;
                if newline_count > 0 {
                    self.current_column = trailing_txt.len() as u32; // TODO: actually base this on grapheme count
                }
                else {
                    self.current_column += txt.len() as u32; // TODO: actually base this on grapheme count
                }
                self.current_byte_offset += bytes_consumed;
                self.remaining_text = &self.remaining_text[bytes_consumed..];
                continue;
            }
            
            // Raw string literal
            else if let Some((0, _)) = re_raw_string_start.find(self.remaining_text) {
                let (newline_count, trailing_txt, txt) = self.lex_raw_string_literal();
                bytes_consumed = txt.len();
                self.tokens.push_back(Token::LIT_RawString(
                    SourceSpan {
                        span: &self.remaining_text[0..bytes_consumed],
                        full_source_text: self.full_text,
                        byte_offset: self.current_byte_offset,
                        line: self.current_line,
                        column: self.current_column,
                    }
                ));
                
                // Handle state updates specially
                self.current_line += newline_count;
                if newline_count > 0 {
                    self.current_column = trailing_txt.len() as u32; // TODO: actually base this on grapheme count
                }
                else {
                    self.current_column += txt.len() as u32; // TODO: actually base this on grapheme count
                }
                self.current_byte_offset += bytes_consumed;
                self.remaining_text = &self.remaining_text[bytes_consumed..];
                continue;
            }
            
            
            // Identifier or keyword
            else if let Some((0, n)) = re_ident_or_keyword.find(self.remaining_text) {
                bytes_consumed = n;
                
                let ss = SourceSpan {
                    span: &self.remaining_text[0..n],
                    full_source_text: self.full_text,
                    byte_offset: self.current_byte_offset,
                    line: self.current_line,
                    column: self.current_column,
                };
                
                self.tokens.push_back(
                    match &self.remaining_text[0..n] {
                        "namespace" => Token::KEY_Namespace(ss),
                        "pub" => Token::KEY_Pub(ss),
                        "unsafe" => Token::KEY_Unsafe(ss),
                        "const" => Token::KEY_Const(ss),
                        "val" => Token::KEY_Val(ss),
                        "var" => Token::KEY_Var(ss),
                        "mut" => Token::KEY_Mut(ss),
                        "ref" => Token::KEY_Ref(ss),
                        "fn" => Token::KEY_Fn(ss),
                        "struct" => Token::KEY_Struct(ss),
                        "enum" => Token::KEY_Enum(ss),
                        "union" => Token::KEY_Union(ss),
                        "trait" => Token::KEY_Trait(ss),
                        "is" => Token::KEY_Is(ss),
                        "if" => Token::KEY_If(ss),
                        "else" => Token::KEY_Else(ss),
                        "loop" => Token::KEY_Loop(ss),
                        "while" => Token::KEY_While(ss),
                        "until" => Token::KEY_Until(ss),
                        "for" => Token::KEY_For(ss),
                        "in" => Token::KEY_In(ss),
                        "break" => Token::KEY_Break(ss),
                        "continue" => Token::KEY_Continue(ss),
                        "return" => Token::KEY_Return(ss),
                        "as" => Token::KEY_As(ss),
                        "alias" => Token::KEY_Alias(ss),
                        "type" => Token::KEY_Type(ss),
                        
                        _ => Token::Identifier(ss),
                    }
                );
            }
            
            // Identifier of a generic parameter
            else if let Some((0, n)) = re_ident_generic.find(self.remaining_text) {
                bytes_consumed = n;
                
                self.tokens.push_back(Token::IdentifierGeneric(
                    SourceSpan {
                        span: &self.remaining_text[0..n],
                        full_source_text: self.full_text,
                        byte_offset: self.current_byte_offset,
                        line: self.current_line,
                        column: self.current_column,
                    }
                ));
            }
            
            // Unknown input text
            else {
                //println!("{:?}", self.remaining_text);
                panic!("Error: unknown text at line {} column {}", self.current_line+1, self.current_column);
            }
            
            // Update state
            self.current_column += bytes_consumed as u32; // TODO: actually base this on grapheme count
            self.current_byte_offset += bytes_consumed;
            self.remaining_text = &self.remaining_text[bytes_consumed..];
        }
        
        return self.tokens;
    }
    
    
    // TODO: error out when the string is not properly closed (i.e. reached EOF)
    fn lex_string_literal(&mut self) -> (u32, &'a str, &'a str) {
        // Find extent of string literal
        let mut last_was_esc = true;
        let mut ending_byte = 0;
        for (b, c) in self.remaining_text.char_indices() {
            ending_byte = b;
            if last_was_esc == true {
                last_was_esc = false;
            }
            else {
                if c == '\\' {
                    last_was_esc = true;
                }
                else if c == '"' {
                    break;
                }
            }
        }
        ending_byte += 1;
        
        // Figure out how many newlines are in the string literal,
        // and what the trailing string is.
        let re_newline = Regex::new(r"(\r\n|\r|\n)").unwrap(); // TODO: re-use same RE from lex().
        let string_text = &self.remaining_text[0..ending_byte];
        let newline_count = re_newline.find_iter(string_text).count() as u32;
        let trailing_text = if let Some(t) = re_newline.split(string_text).last() {t} else {string_text};
        
        return (newline_count, trailing_text, string_text);
    }
    
    
    // TODO: error out when the string is not properly closed (i.e. reached EOF)
    fn lex_raw_string_literal(&mut self) -> (u32, &'a str, &'a str) {
        // Find extent of string literal
        let mut stage = 0i32;
        let mut start_tick_count = 0;
        let mut tick_count = 0;
        let mut ending_byte = 0;
        for (b, c) in self.remaining_text.char_indices() {
            ending_byte = b;
            if stage == 0 {
                // Get the starting tick count
                if c == '\'' {
                    start_tick_count += 1;
                }
                else {
                    stage = 1;
                }
            }
            else if stage == 1 {
                // Look for double-quotes
                if c == '"' {
                    stage = 2;
                }
            }
            else if stage == 2 {
                // Count closing ticks
                if c == '\'' {
                    tick_count += 1;
                    if tick_count == start_tick_count {
                        break;
                    }
                }
                else {
                    tick_count = 0;
                    stage = 1;
                }
            }
        }
        
        ending_byte += 1;
        
        // Figure out how many newlines are in the string literal,
        // and what the trailing string is.
        let re_newline = Regex::new(r"(\r\n|\r|\n)").unwrap(); // TODO: re-use same RE from lex().
        let string_text = &self.remaining_text[0..ending_byte];
        let newline_count = re_newline.find_iter(string_text).count() as u32;
        let trailing_text = if let Some(t) = re_newline.split(string_text).last() {t} else {string_text};
        
        return (newline_count, trailing_text, string_text);
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use source_span::SourceSpan;
    use token::Token;
    
    #[test]
    fn idents_and_keywords_1() {
        let text = "var hello";
        let tokens = lex_str(text);
        
        assert_eq!(tokens[0], Token::KEY_Var(
            SourceSpan {
                span: "var",
                full_source_text: text,
                byte_offset: 0,
                line: 0,
                column: 0
            }
        ));
        assert_eq!(tokens[1], Token::Identifier(
            SourceSpan {
                span: "hello",
                full_source_text: text,
                byte_offset: 4,
                line: 0,
                column: 4
            }
        ));
        assert_eq!(tokens[2], Token::EOF);
    }
    
    #[test]
    fn idents_and_keywords_2() {
        let text = "var a";
        let tokens = lex_str(text);
        
        assert_eq!(tokens[0], Token::KEY_Var(
            SourceSpan {
                span: "var",
                full_source_text: text,
                byte_offset: 0,
                line: 0,
                column: 0
            }
        ));
        assert_eq!(tokens[1], Token::Identifier(
            SourceSpan {
                span: "a",
                full_source_text: text,
                byte_offset: 4,
                line: 0,
                column: 4
            }
        ));
        assert_eq!(tokens[2], Token::EOF);
    }
    
    #[test]
    fn newlines() {
        let text = "var\n \n   \n hello";
        let tokens = lex_str(text);
        
        assert_eq!(tokens[0], Token::KEY_Var(
            SourceSpan {
                span: "var",
                full_source_text: text,
                byte_offset: 0,
                line: 0,
                column: 0
            }
        ));
        assert_eq!(tokens[1], Token::NewLine(
            SourceSpan {
                span: "\n",
                full_source_text: text,
                byte_offset: 3,
                line: 0,
                column: 3
            }
        ));
        assert_eq!(tokens[2], Token::NewLine(
            SourceSpan {
                span: "\n",
                full_source_text: text,
                byte_offset: 5,
                line: 1,
                column: 1
            }
        ));
        assert_eq!(tokens[3], Token::NewLine(
            SourceSpan {
                span: "\n",
                full_source_text: text,
                byte_offset: 9,
                line: 2,
                column: 3
            }
        ));
        assert_eq!(tokens[4], Token::Identifier(
            SourceSpan {
                span: "hello",
                full_source_text: text,
                byte_offset: 11,
                line: 3,
                column: 1
            }
        ));
        assert_eq!(tokens[5], Token::EOF);
    }
    
    #[test]
    fn punctuation() {
        let text = "{}()[]@.,:`$";
        let tokens = lex_str(text);
        
        assert_eq!(tokens[0], Token::LCurly(SourceSpan {span: "{", full_source_text: text, byte_offset: 0, line: 0, column: 0}));
        assert_eq!(tokens[1], Token::RCurly(SourceSpan {span: "}", full_source_text: text, byte_offset: 1, line: 0, column: 1}));
        assert_eq!(tokens[2], Token::LParen(SourceSpan {span: "(", full_source_text: text, byte_offset: 2, line: 0, column: 2}));
        assert_eq!(tokens[3], Token::RParen(SourceSpan {span: ")", full_source_text: text, byte_offset: 3, line: 0, column: 3}));
        assert_eq!(tokens[4], Token::LSquare(SourceSpan {span: "[", full_source_text: text, byte_offset: 4, line: 0, column: 4}));
        assert_eq!(tokens[5], Token::RSquare(SourceSpan {span: "]", full_source_text: text, byte_offset: 5, line: 0, column: 5}));
        assert_eq!(tokens[6], Token::At(SourceSpan {span: "@", full_source_text: text, byte_offset: 6, line: 0, column: 6}));
        assert_eq!(tokens[7], Token::Period(SourceSpan {span: ".", full_source_text: text, byte_offset: 7, line: 0, column: 7}));
        assert_eq!(tokens[8], Token::Comma(SourceSpan {span: ",", full_source_text: text, byte_offset: 8, line: 0, column: 8}));
        assert_eq!(tokens[9], Token::Colon(SourceSpan {span: ":", full_source_text: text, byte_offset: 9, line: 0, column: 9}));
        assert_eq!(tokens[10], Token::BackTick(SourceSpan {span: "`", full_source_text: text, byte_offset: 10, line: 0, column: 10}));
        assert_eq!(tokens[11], Token::Dollar(SourceSpan {span: "$", full_source_text: text, byte_offset: 11, line: 0, column: 11}));
        assert_eq!(tokens[12], Token::EOF);
    }
    
    #[test]
    fn operator() {
        let text = "- + / * % | & ! ~ ++-*&|%";
        let tokens = lex_str(text);
        
        assert_eq!(tokens[0], Token::Operator(SourceSpan {span: "-", full_source_text: text, byte_offset: 0, line: 0, column: 0}));
        assert_eq!(tokens[1], Token::Operator(SourceSpan {span: "+", full_source_text: text, byte_offset: 2, line: 0, column: 2}));
        assert_eq!(tokens[2], Token::Operator(SourceSpan {span: "/", full_source_text: text, byte_offset: 4, line: 0, column: 4}));
        assert_eq!(tokens[3], Token::Operator(SourceSpan {span: "*", full_source_text: text, byte_offset: 6, line: 0, column: 6}));
        assert_eq!(tokens[4], Token::Operator(SourceSpan {span: "%", full_source_text: text, byte_offset: 8, line: 0, column: 8}));
        assert_eq!(tokens[5], Token::Operator(SourceSpan {span: "|", full_source_text: text, byte_offset: 10, line: 0, column: 10}));
        assert_eq!(tokens[6], Token::Operator(SourceSpan {span: "&", full_source_text: text, byte_offset: 12, line: 0, column: 12}));
        assert_eq!(tokens[7], Token::Operator(SourceSpan {span: "!", full_source_text: text, byte_offset: 14, line: 0, column: 14}));
        assert_eq!(tokens[8], Token::Operator(SourceSpan {span: "~", full_source_text: text, byte_offset: 16, line: 0, column: 16}));
        assert_eq!(tokens[9], Token::Operator(SourceSpan {span: "++-*&|%", full_source_text: text, byte_offset: 18, line: 0, column: 18}));
        assert_eq!(tokens[10], Token::EOF);
    }
    
    #[test]
    fn ints_and_reals() {
        let text = "123 12.3";
        let tokens = lex_str(text);
        
        assert_eq!(tokens[0], Token::LIT_Int(
            SourceSpan {
                span: "123",
                full_source_text: text,
                byte_offset: 0,
                line: 0,
                column: 0
            }
        ));
        assert_eq!(tokens[1], Token::LIT_Real(
            SourceSpan {
                span: "12.3",
                full_source_text: text,
                byte_offset: 4,
                line: 0,
                column: 4
            }
        ));
        assert_eq!(tokens[2], Token::EOF);
    }
    
    #[test]
    fn comments() {
        let text = "var hello# How's it going?\n";
        let tokens = lex_str(text);
        
        assert_eq!(tokens[0], Token::KEY_Var(
            SourceSpan {
                span: "var",
                full_source_text: text,
                byte_offset: 0,
                line: 0,
                column: 0
            }
        ));
        assert_eq!(tokens[1], Token::Identifier(
            SourceSpan {
                span: "hello",
                full_source_text: text,
                byte_offset: 4,
                line: 0,
                column: 4
            }
        ));
        assert_eq!(tokens[2], Token::NewLine(
            SourceSpan {
                span: "\n",
                full_source_text: text,
                byte_offset: 26,
                line: 0,
                column: 26
            }
        ));
        assert_eq!(tokens[3], Token::EOF);
    }
    
    #[test]
    fn doc_comments() {
        let text = "var hello#: How's it going?\n";
        let tokens = lex_str(text);
        
        assert_eq!(tokens[0], Token::KEY_Var(
            SourceSpan {
                span: "var",
                full_source_text: text,
                byte_offset: 0,
                line: 0,
                column: 0
            }
        ));
        assert_eq!(tokens[1], Token::Identifier(
            SourceSpan {
                span: "hello",
                full_source_text: text,
                byte_offset: 4,
                line: 0,
                column: 4
            }
        ));
        assert_eq!(tokens[2], Token::DocComment(
            SourceSpan {
                span: "#: How's it going?",
                full_source_text: text,
                byte_offset: 9,
                line: 0,
                column: 9
            }
        ));
        assert_eq!(tokens[3], Token::NewLine(
            SourceSpan {
                span: "\n",
                full_source_text: text,
                byte_offset: 27,
                line: 0,
                column: 27
            }
        ));
        assert_eq!(tokens[4], Token::EOF);

    }
    
    #[test]
    fn string_literal_1() {
        let text = r#"var"Suddenly there's \"a string!"hello"#;
        let tokens = lex_str(text);
        
        assert_eq!(tokens[0], Token::KEY_Var(
            SourceSpan {
                span: "var",
                full_source_text: text,
                byte_offset: 0,
                line: 0,
                column: 0
            }
        ));
        assert_eq!(tokens[1], Token::LIT_String(
            SourceSpan {
                span: r#""Suddenly there's \"a string!""#,
                full_source_text: text,
                byte_offset: 3,
                line: 0,
                column: 3
            }
        ));
        assert_eq!(tokens[2], Token::Identifier(
            SourceSpan {
                span: "hello",
                full_source_text: text,
                byte_offset: 33,
                line: 0,
                column: 33
            }
        ));
        assert_eq!(tokens[3], Token::EOF);
    }
     
    #[test]
    fn raw_string_literal_1() {
        let text = r#"var'"Suddenly there's "a raw string!"'hello"#;
        let tokens = lex_str(text);
        
        assert_eq!(tokens[0], Token::KEY_Var(
            SourceSpan {
                span: "var",
                full_source_text: text,
                byte_offset: 0,
                line: 0,
                column: 0
            }
        ));
        assert_eq!(tokens[1], Token::LIT_RawString(
            SourceSpan {
                span: r#"'"Suddenly there's "a raw string!"'"#,
                full_source_text: text,
                byte_offset: 3,
                line: 0,
                column: 3
            }
        ));
        assert_eq!(tokens[2], Token::Identifier(
            SourceSpan {
                span: "hello",
                full_source_text: text,
                byte_offset: 38,
                line: 0,
                column: 38
            }
        ));
        assert_eq!(tokens[3], Token::EOF);
    }
    
    #[test]
    fn raw_string_literal_2() {
        let text = r#"var''"Suddenly there's "'a raw string!"''hello"#;
        let tokens = lex_str(text);
        
        assert_eq!(tokens[0], Token::KEY_Var(
            SourceSpan {
                span: "var",
                full_source_text: text,
                byte_offset: 0,
                line: 0,
                column: 0
            }
        ));
        assert_eq!(tokens[1], Token::LIT_RawString(
            SourceSpan {
                span: r#"''"Suddenly there's "'a raw string!"''"#,
                full_source_text: text,
                byte_offset: 3,
                line: 0,
                column: 3
            }
        ));
        assert_eq!(tokens[2], Token::Identifier(
            SourceSpan {
                span: "hello",
                full_source_text: text,
                byte_offset: 41,
                line: 0,
                column: 41
            }
        ));
        assert_eq!(tokens[3], Token::EOF);
    }
}