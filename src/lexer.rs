use crate::token::{lookup_identifier, Token, TokenKind};

struct Lexer {
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
            '=' => Lexer::new_token(TokenKind::Assign, self.ch),
            '!' => Lexer::new_token(TokenKind::Bang, self.ch),
            '-' => Lexer::new_token(TokenKind::Minus, self.ch),
            '/' => Lexer::new_token(TokenKind::Slash, self.ch),
            '*' => Lexer::new_token(TokenKind::Asterisk, self.ch),
            '<' => Lexer::new_token(TokenKind::LessThan, self.ch),
            '>' => Lexer::new_token(TokenKind::GreaterThan, self.ch),
            '\0' => Lexer::new_token(TokenKind::Eof, '\0'),
            _ => {
                return if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = lookup_identifier(&literal);

                    Token { kind, literal }
                } else if Lexer::is_num(self.ch) {
                    let literal = self.read_num();
                    let kind = TokenKind::Integer;

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

    fn new_token(kind: TokenKind, ch: char) -> Token {
        Token {
            kind,
            literal: ch.to_string(),
        }
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
    use crate::token::{Token, TokenKind};

    use super::Lexer;

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
                kind: TokenKind::Integer,
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
                kind: TokenKind::Integer,
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
                kind: TokenKind::Integer,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            // 2 < 3 > 8;
            Token {
                kind: TokenKind::Integer,
                literal: "2".to_string(),
            },
            Token {
                kind: TokenKind::LessThan,
                literal: "<".to_string(),
            },
            Token {
                kind: TokenKind::Integer,
                literal: "3".to_string(),
            },
            Token {
                kind: TokenKind::GreaterThan,
                literal: ">".to_string(),
            },
            Token {
                kind: TokenKind::Integer,
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
}
