use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT(isize),

    LET,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    COMMA,
    SEMICOLON,
    FUNCTION,
    IF,
    ELSE,

    LT,
    GT,
    LTEQ,
    GTEQ,

    TRUE,
    FALSE,

    RETURN,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    UNKNOWN,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

lazy_static::lazy_static! {
    pub static ref IDENTS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("let", TokenType::LET);
        map.insert("fn", TokenType::FUNCTION);
        map.insert("if", TokenType::IF);
        map.insert("else", TokenType::ELSE);
        map.insert("return", TokenType::RETURN);
        map.insert("true", TokenType::TRUE);
        map.insert("false", TokenType::FALSE);
        map
    };
}
#[derive(PartialEq, Debug)]
pub struct Lexer {
    pub input: &'static str,
    pub ch: char,
    pub ch_pos: usize,
    pub reading_position: usize,
}

impl Lexer {
    pub fn new(inp: &'static str) -> Self {
        Lexer {
            input: inp,
            ch: inp.chars().nth(0).unwrap(),
            ch_pos: 0,
            reading_position: 1,
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token: Token = match self.ch {
            '=' => Self::new_token(TokenType::ASSIGN, self.ch),
            ';' => Self::new_token(TokenType::SEMICOLON, self.ch),
            '(' => Self::new_token(TokenType::LPAREN, self.ch),
            ')' => Self::new_token(TokenType::RPAREN, self.ch),
            ',' => Self::new_token(TokenType::COMMA, self.ch),
            '+' => Self::new_token(TokenType::PLUS, self.ch),
            '{' => Self::new_token(TokenType::LBRACE, self.ch),
            '}' => Self::new_token(TokenType::RBRACE, self.ch),
            '-' => Self::new_token(TokenType::MINUS, self.ch),
            '!' => Self::new_token(TokenType::BANG, self.ch),
            '*' => Self::new_token(TokenType::ASTERISK, self.ch),
            '/' => Self::new_token(TokenType::SLASH, self.ch),
            '<' => Self::new_token(TokenType::LT, self.ch),
            '>' => Self::new_token(TokenType::GT, self.ch),
            '[' => Self::new_token(TokenType::LBRACKET, self.ch),
            ']' => Self::new_token(TokenType::RBRACKET, self.ch),
            '\0' => Self::new_token(TokenType::EOF, self.ch),
            ch if ch.to_string() == "" => Self::new_token(TokenType::EOF, self.ch),
            ch if ch as usize == 0 => Self::new_token(TokenType::EOF, self.ch),
            ch => {
                if ch.is_ascii_alphabetic() {
                    let lit = self.read_ident();
                    Token {
                        token_type: if let Some(ident) = Lexer::determine_kw(&lit) {
                            ident.clone()
                        } else {
                            TokenType::IDENT(lit.clone())
                        },
                        literal: lit,
                    }
                } else if ch.is_ascii_digit() {
                    let dig = self.read_number().parse::<isize>().unwrap();
                    Token {
                        token_type: TokenType::INT(dig),
                        literal: dig.to_string(),
                    }
                } else {
                    Token {
                        token_type: TokenType::IDENT(ch.to_string()),
                        literal: ch.to_string(),
                    }
                }
            }
        };

        self.read_char();

        token
    }

    pub fn new_token(tok_type: TokenType, ch: char) -> Token {
        Token {
            token_type: tok_type,
            literal: ch.to_string(),
        }
    }

    pub fn read_char(&mut self) {
        if self.reading_position as usize >= self.input.len() {
            self.ch = '\0'
        } else {
            self.ch = self.input.chars().nth(self.reading_position).unwrap();
        }

        self.ch_pos = self.reading_position;
        self.reading_position += 1;
    }

    pub fn read_ident(&mut self) -> String {
        let pos = self.ch_pos;

        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }

        return self.input.chars().collect::<Vec<char>>()[pos..self.ch_pos]
            .iter()
            .collect();
    }

    pub fn read_number(&mut self) -> String {
        let pos = self.ch_pos;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        return self.input.chars().collect::<Vec<char>>()[pos..self.ch_pos]
            .iter()
            .collect();
    }

    pub fn determine_kw(ident: &str) -> Option<&TokenType> {
        if IDENTS.contains_key(ident) {
            IDENTS.get(ident)
        } else {
            None
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
