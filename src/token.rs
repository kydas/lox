use crate::token_type::TokenType;

pub struct Token<T> {
    tok_type: TokenType,
    lexeme: String,
    literal: T,
    line: u32,
}

impl <T: ToString> Token<T> {
    fn toString(&self) -> String {
        return String::from(self.tok_type.to_string() + " " + &self.lexeme.to_string() + " " + &self.literal.to_string());
    }
}