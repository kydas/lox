use token::Token;
use token_type::TokenType;

struct Scanner {
    source: String,
    tokens: Vec<Token>
    start: u32 = 0,
    current: u32 = 0,
    line: u32 = 1,
}

impl Scanner {
    fn scan_tokens(&mut self) -> Vec<Token> {
        while (!self.is_at_end()) {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push({TokenType::EOF, "", None, self.line});
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

