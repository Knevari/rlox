use crate::tokens::{Token, TokenType, Literal};
use crate::error::Error;

pub struct Scanner {
    start: usize,
    current: usize,
    line: usize,
    source: String,
    tokens: Vec<Token>,
    had_error: bool
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            had_error: false
        }
    }

    fn peek(&self) -> char {
        if self.is_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn advance(&mut self) -> char {
        let current_index: usize = self.current.try_into().unwrap();
        let c = self.source.chars().nth(current_index).unwrap();
        self.current += 1;
        return c;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(
            Token::new(
                token_type, 
                text.to_string(), 
                literal,
                self.line
            )
        )
    }

    fn add_token_with_no_value(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::NoValue)
    }

    fn add_token_if_match_next(&mut self, c: char, t: TokenType, k: TokenType) {
        if self.match_next(c) {
            self.add_token_with_no_value(t);
        } else {
            self.add_token_with_no_value(k);
        }
    }

    fn match_next(&mut self, c: char) -> bool {
        let dc = self.source.chars().nth(self.current + 1).unwrap();

        if self.is_end() || dc != c {
            return false;
        }

        self.current += 1;
        return true;
    }
    
    fn skip_line(&mut self) {
        while self.peek() != '\n' && !self.is_end() {
            self.advance();
        }
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            // Single character
            '(' => self.add_token_with_no_value(TokenType::LeftParen),
            ')' => self.add_token_with_no_value(TokenType::RightParen),
            '{' => self.add_token_with_no_value(TokenType::LeftBrace),
            '}' => self.add_token_with_no_value(TokenType::RightBrace),
            ',' => self.add_token_with_no_value(TokenType::Comma),
            '.' => self.add_token_with_no_value(TokenType::Dot),
            '-' => self.add_token_with_no_value(TokenType::Minus),
            '+' => self.add_token_with_no_value(TokenType::Plus),
            ';' => self.add_token_with_no_value(TokenType::Semicolon),
            '*' => self.add_token_with_no_value(TokenType::Star),
            // Maybe second character
            '!' => self.add_token_if_match_next('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.add_token_if_match_next('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.add_token_if_match_next('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.add_token_if_match_next('=', TokenType::GreaterEqual, TokenType::Greater),
            // A slash can be either division or a comment
            '/' => {
                if self.match_next('/') {
                    self.skip_line();
                } else {
                    self.add_token_with_no_value(TokenType::Slash)
                }
            },
            // Ignore whitespaces
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            // Strings
            '"' => self.read_string(),
            other => {
                if other.is_digit(10) {
                    self.read_number();
                } else if other.is_alphabetic() {
                    self.read_identifier();
                } else {
                    Error::complain(self.line, "Unexpected character".to_string())
                }
            }
        }
    }

    fn read_number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value: u64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number, Literal::Number(value))
    }

    fn read_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let kw_type = TokenType::get_keyword_type(text);

        if let TokenType::NonExistent = kw_type{
            self.add_token_with_no_value(TokenType::Identifier);
        } else {
            self.add_token_with_no_value(kw_type);
        }
    }

    fn read_string(&mut self) {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            Error::complain(self.line, "Unterminated string".to_string());
        }

        self.advance();

        let value = self.source[self.start+1..self.current-1].to_string();
        self.add_token(TokenType::String, Literal::String(value))
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Literal::NoValue,
            self.line
        ));

        &self.tokens
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len().try_into().unwrap()
    }
}