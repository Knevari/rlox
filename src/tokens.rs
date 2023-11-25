use std::fmt;

#[derive(Debug)]

pub enum Literal {
  NoValue,
  String(String),
  Number(u64)
}

impl fmt::Display for Literal {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
pub enum TokenType {
  // Single character tokens
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals
  Identifier, String, Number,

  // Keywords
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof,

  NonExistent
}

impl TokenType {
  pub fn get_keyword_type(s: &str) -> TokenType {
    match s {
      "and" => TokenType::And,
      "class" => TokenType::Class,
      "else" => TokenType::Else,
      "false" => TokenType::False,
      "fun" => TokenType::Fun,
      "for" => TokenType::For,
      "if" => TokenType::If,
      "nil" => TokenType::Nil,
      "or" => TokenType::Or,
      "print" => TokenType::Print,
      "return" => TokenType::Return,
      "super" => TokenType::Super,
      "this" => TokenType::This,
      "true" => TokenType::True,
      "var" => TokenType::Var,
      "while" => TokenType::While,
      _ => TokenType::NonExistent
    }
  }
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub struct Token {
  token_type: TokenType,
  lexeme: String,
  literal: Literal,
  line: usize
}

impl Token {
  pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Token{
    Token {
      token_type,
      lexeme,
      literal,
      line
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {} {} at line ${}", self.token_type, self.literal, self.lexeme, self.line)
  }
}
