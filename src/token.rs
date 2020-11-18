use crate::token_type::TokenType;
use crate::scanner::Null;

#[derive(Clone)]
pub struct Token {
    pub tok_type: TokenType,
    pub lexeme: String,
    pub literal: Box<dyn Literal>,
    pub line: u32,
}



pub trait Literal: LiteralClone + std::fmt::Display {}

impl Literal for bool {}


pub trait LiteralClone {
    fn clone_box(&self) -> Box<dyn Literal>;
}

impl<T> LiteralClone for T where T: 'static + Literal + Clone {
    fn clone_box(&self) -> Box<dyn Literal> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Literal> {
    fn clone(&self) -> Box<dyn Literal> {
        self.clone_box()
    }
}

impl Token {

    pub fn to_string(&self) -> String {
        return String::from(self.tok_type.to_string() + " " + &self.lexeme.to_string() + " " + &self.literal.to_string());
    }
}

// This doesn't really need to exit anymore, could just write a method now that literal is a box
#[macro_export]
macro_rules! token {
    ( $t:expr, $lx:expr, $lit:expr, $ln:expr) => {
        Token {
            tok_type: $t,
            lexeme: $lx,
            literal: $lit,
            line: $ln
        }
    };
}