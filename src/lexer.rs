use crate::token::{Token, TokenKind};

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
        let token = match self.ch {
            ';' => Lexer::new_token(TokenKind::Semicolon, self.ch),
            ',' => Lexer::new_token(TokenKind::Comma, self.ch),
            '(' => Lexer::new_token(TokenKind::LeftParen, self.ch),
            ')' => Lexer::new_token(TokenKind::RightParen, self.ch),
            '{' => Lexer::new_token(TokenKind::LeftBrace, self.ch),
            '}' => Lexer::new_token(TokenKind::RightBrace, self.ch),
            '+' => Lexer::new_token(TokenKind::Plus, self.ch),
            '=' => Lexer::new_token(TokenKind::Assign, self.ch),
            '\0' => Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            },
            _ => Lexer::new_token(TokenKind::Illegal, self.ch)
        };

        self.read_char();

        token
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
}
