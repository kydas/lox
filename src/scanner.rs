use crate::token::Token;
use crate::token_type::TokenType;
use std::fmt;

#[macro_use]

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {

    fn new(source: String) -> Scanner {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    // TODO: properly implement this now that token is done
    fn scan_tokens(&mut self) -> Vec<Token> {
        // while (!self.is_at_end()) {
        //     self.start = self.current;
        //     self.scan_token();
        // }

        let lit = Box::new(false);
        
        let tok = token!(TokenType::False, String::from("false"), lit, self.line);
        // let tok = Token {
        //     tok_type: TokenType::False,
        //     lexeme: String::from("false"),
        //     literal: lit,
        //     line: self.line
        // };

        println!("{}",tok.to_string());

        self.tokens.push(tok);
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }
}

pub struct Null;

impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "null")
    }
}

