use crate::token::{lookup_identifier, Token, TokenKind};

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    read_pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            pos: 0,
            read_pos: 0,
            ch: Default::default(),
        };

        lexer.read_char();

        lexer
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            ';' => Lexer::new_token(TokenKind::Semicolon, self.ch),
            ',' => Lexer::new_token(TokenKind::Comma, self.ch),
            '(' => Lexer::new_token(TokenKind::LeftParen, self.ch),
            ')' => Lexer::new_token(TokenKind::RightParen, self.ch),
            '{' => Lexer::new_token(TokenKind::LeftBrace, self.ch),
            '}' => Lexer::new_token(TokenKind::RightBrace, self.ch),
            '+' => Lexer::new_token(TokenKind::Plus, self.ch),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        kind: TokenKind::Eq,
                        literal: String::from("=="),
                    }
                } else {
                    Lexer::new_token(TokenKind::Assign, self.ch)
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        kind: TokenKind::NotEq,
                        literal: String::from("!="),
                    }
                } else {
                    Lexer::new_token(TokenKind::Bang, self.ch)
                }
            }
            '-' => Lexer::new_token(TokenKind::Minus, self.ch),
            '/' => {
                if self.peek_char() == '/' {
                    self.skip_comment();
                    return self.next();
                } else {
                    Lexer::new_token(TokenKind::Slash, self.ch)
                }
            }
            '*' => Lexer::new_token(TokenKind::Asterisk, self.ch),
            '<' => Lexer::new_token(TokenKind::LessThan, self.ch),
            '>' => Lexer::new_token(TokenKind::GreaterThan, self.ch),
            '"' => {
                self.read_char();

                let literal = self.read_str();
                Token {
                    kind: TokenKind::Str,
                    literal,
                }
            }
            '\0' => Lexer::new_token(TokenKind::Eof, '\0'),
            _ => {
                return if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = lookup_identifier(&literal);

                    Token { kind, literal }
                } else if Lexer::is_num(self.ch) {
                    let literal = self.read_num();
                    let kind = TokenKind::Int;

                    Token { kind, literal }
                } else {
                    Lexer::new_token(TokenKind::Illegal, self.ch)
                }
            }
        };

        self.read_char();

        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn skip_comment(&mut self) {
        while self.ch != '\n' {
            self.read_char();
        }
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn is_num(ch: char) -> bool {
        ch.is_numeric()
    }

    fn read_num(&mut self) -> String {
        let mut num = String::new();

        while Lexer::is_num(self.ch) {
            num.push(self.ch);
            self.read_char();
        }

        num
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while Lexer::is_letter(self.ch) {
            identifier.push(self.ch);
            self.read_char();
        }

        identifier
    }

    fn read_str(&mut self) -> String {
        let mut identifier = String::new();
        while self.ch != '"' {
            identifier.push(self.ch);
            self.read_char();
        }

        identifier
    }

    fn new_token(kind: TokenKind, ch: char) -> Token {
        Token {
            kind,
            literal: ch.to_string(),
        }
    }

    fn peek_char(&self) -> char {
        return if self.read_pos >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_pos]
        };
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = '\0'; // ascii eof
        } else {
            self.ch = self.input[self.read_pos];
        }

        self.pos = self.read_pos;
        self.read_pos += 1;
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_strings() {
        let input = r#"
let name = "John";
"#;

        let expected: Vec<Token> = vec![
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "name".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Str,
                literal: "John".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (index, expected_token) in expected.into_iter().enumerate() {
            let next_token = lexer.next();
            assert_eq!(
                expected_token.kind, next_token.kind,
                "Index={index} incorrect token, Expected={}, Got={}",
                expected_token.kind, next_token.kind
            );

            assert_eq!(
                expected_token.literal, next_token.literal,
                "Index={index} incorrect literal, Expected={}, Got={}",
                expected_token.literal, next_token.literal
            );
        }
    }

    #[test]
    fn test_comments_ignored() {
        let input = r#"
// Comments should be ignored!
let is_logged_in = true;
"#;

        let expected: Vec<Token> = vec![
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "is_logged_in".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::True,
                literal: "true".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (index, expected_token) in expected.into_iter().enumerate() {
            let next_token = lexer.next();
            assert_eq!(
                expected_token.kind, next_token.kind,
                "Index={index} incorrect token, Expected={}, Got={}",
                expected_token.kind, next_token.kind
            );

            assert_eq!(
                expected_token.literal, next_token.literal,
                "Index={index} incorrect literal, Expected={}, Got={}",
                expected_token.literal, next_token.literal
            );
        }
    }

    #[test]
    fn test_parse_basic_script() {
        let input = r#"let one = 1;
let three = 3;

let add = fn(x, y) {
    x + y
}

let result = add(one, three);
"#;

        let expected: Vec<Token> = vec![
            // let one = 1
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "one".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "1".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            // let three = 3
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "three".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "3".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            // let add = fn(x, y) { x + y }
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Fn,
                literal: "fn".to_string(),
            },
            Token {
                kind: TokenKind::LeftParen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
            },
            // let result = add(one, three)
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "result".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::LeftParen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "one".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "three".to_string(),
            },
            Token {
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (index, expected_token) in expected.into_iter().enumerate() {
            let next_token = lexer.next();
            assert_eq!(
                expected_token.kind, next_token.kind,
                "Index={index} incorrect token, Expected={}, Got={}",
                expected_token.kind, next_token.kind
            );

            assert_eq!(
                expected_token.literal, next_token.literal,
                "Index={index} incorrect literal, Expected={}, Got={}",
                expected_token.literal, next_token.literal
            );
        }
    }

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let expected: Vec<Token> = vec![
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::LeftParen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (index, expected_token) in expected.into_iter().enumerate() {
            let next_token = lexer.next();
            assert_eq!(
                expected_token.kind, next_token.kind,
                "Index={index} incorrect token, Expected={}, Got={}",
                expected_token.kind, next_token.kind
            );

            assert_eq!(
                expected_token.literal, next_token.literal,
                "Index={index} incorrect literal, Expected={}, Got={}",
                expected_token.literal, next_token.literal
            );
        }
    }

    #[test]
    fn test_additional_tokens() {
        let input = r#"
        !-/*5;
        2 < 3 > 8;
        "#;

        let expected: Vec<Token> = vec![
            // !-/*5;
            Token {
                kind: TokenKind::Bang,
                literal: "!".to_string(),
            },
            Token {
                kind: TokenKind::Minus,
                literal: "-".to_string(),
            },
            Token {
                kind: TokenKind::Slash,
                literal: "/".to_string(),
            },
            Token {
                kind: TokenKind::Asterisk,
                literal: "*".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            // 2 < 3 > 8;
            Token {
                kind: TokenKind::Int,
                literal: "2".to_string(),
            },
            Token {
                kind: TokenKind::LessThan,
                literal: "<".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "3".to_string(),
            },
            Token {
                kind: TokenKind::GreaterThan,
                literal: ">".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "8".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (index, expected_token) in expected.into_iter().enumerate() {
            let next_token = lexer.next();
            assert_eq!(
                expected_token.kind, next_token.kind,
                "Index={index} incorrect token, Expected={}, Got={}",
                expected_token.kind, next_token.kind
            );

            assert_eq!(
                expected_token.literal, next_token.literal,
                "Index={index} incorrect literal, Expected={}, Got={}",
                expected_token.literal, next_token.literal
            );
        }
    }

    #[test]
    fn test_if_else_statement() {
        let input = r#"
        if (4 > 2) {
            return true;
        } else {
            return false;
        }
        "#;

        let expected: Vec<Token> = vec![
            // if (4 > 2) { return true; } else { return false; }
            Token {
                kind: TokenKind::If,
                literal: "if".to_string(),
            },
            Token {
                kind: TokenKind::LeftParen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "4".to_string(),
            },
            Token {
                kind: TokenKind::GreaterThan,
                literal: ">".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "2".to_string(),
            },
            Token {
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Return,
                literal: "return".to_string(),
            },
            Token {
                kind: TokenKind::True,
                literal: "true".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Else,
                literal: "else".to_string(),
            },
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Return,
                literal: "return".to_string(),
            },
            Token {
                kind: TokenKind::False,
                literal: "false".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (index, expected_token) in expected.into_iter().enumerate() {
            let next_token = lexer.next();
            assert_eq!(
                expected_token.kind, next_token.kind,
                "Index={index} incorrect token, Expected={}, Got={}",
                expected_token.kind, next_token.kind
            );

            assert_eq!(
                expected_token.literal, next_token.literal,
                "Index={index} incorrect literal, Expected={}, Got={}",
                expected_token.literal, next_token.literal
            );
        }
    }

    #[test]
    fn test_eq_eq_and_not_eq() {
        let input = r#"
        1 == 1;
        2 != 1;
        "#;

        let expected: Vec<Token> = vec![
            // 1 == 1;
            Token {
                kind: TokenKind::Int,
                literal: "1".to_string(),
            },
            Token {
                kind: TokenKind::Eq,
                literal: "==".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "1".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            // 2 != 1;
            Token {
                kind: TokenKind::Int,
                literal: "2".to_string(),
            },
            Token {
                kind: TokenKind::NotEq,
                literal: "!=".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "1".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (index, expected_token) in expected.into_iter().enumerate() {
            let next_token = lexer.next();
            assert_eq!(
                expected_token.kind, next_token.kind,
                "Index={index} incorrect token, Expected={}, Got={}",
                expected_token.kind, next_token.kind
            );

            assert_eq!(
                expected_token.literal, next_token.literal,
                "Index={index} incorrect literal, Expected={}, Got={}",
                expected_token.literal, next_token.literal
            );
        }
    }
}
