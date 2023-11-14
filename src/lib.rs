pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::lexer::lex::*;
    #[test]
    fn test_creation() {
        let expected = Lexer {
            input: "let x = 5;",
            ch: 'l',
            ch_pos: 0,
            reading_position: 1,
        };

        let res = Lexer::new("let x = 5;");
        assert_eq!(expected, res);
    }

    #[test]
    fn read_char() {
        let mut lex = Lexer::new("let x = 5;");
        lex.read_char();
        assert_eq!('e', lex.ch);
    }

    #[test]
    fn read_multiple() {
        let mut lex = Lexer::new("let x = 5;");
        for _ in 0..6 {
            lex.read_char();
        }

        assert_eq!('=', lex.ch)
    }

    #[test]
    fn next_token_test() {
        let mut lex = Lexer::new("(){}=*+-,;");
        let expected = Token {
            token_type: TokenType::LPAREN,
            literal: "(".to_string(),
        };
        let actual = lex.next_token();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_read_ident() {
        let mut lex = Lexer::new("let");
        let expected = "let";
        assert_eq!(expected, lex.read_ident())
    }

    #[test]
    fn test_determine_kw() {
        let mut lex = Lexer::new("let five");
        let expected = Token {
            token_type: TokenType::IDENT("five".to_string()),
            literal: "five".into(),
        };
        ..lex.next_token();
        let res = lex.next_token();
        assert_eq!(expected, res);
    }

    #[test]
    fn test_read_number() {
        let mut lex = Lexer::new("123");
        let expected = Token {
            token_type: TokenType::INT(123),
            literal: "123".into(),
        };

        assert_eq!(expected, lex.next_token());
    }

    fn final_boss() {
        let input = r#"
            let five = 5;
            let ten = ten;

            let add = fn(x, y) {
                return x + y;
            };

            let result = add(five, ten);
        "#;

        let expected = vec![
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("five".to_string()),
                literal: "five".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::INT(5),
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("ten".to_string()),
                literal: "ten".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::INT(10),
                literal: "10".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("add".to_string()),
                literal: "add".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::FUNCTION,
                literal: "fn".to_string(),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("x".to_string()),
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("y".to_string()),
                literal: "y".to_string(),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::RETURN,
                literal: "return".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("x".to_string()),
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::PLUS,
                literal: "+".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("y".to_string()),
                literal: "y".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("result".to_string()),
                literal: "result".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("add".to_string()),
                literal: "add".to_string(),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("five".to_string()),
                literal: "five".to_string(),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("ten".to_string()),
                literal: "ten".to_string(),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
        ];

        let mut lex = Lexer::new(input);
        let mut actual = Vec::new();
        for _ in 0..expected.len() {
            actual.push(lex.next_token());
        }

        assert_eq!(expected, actual);
    }
}
