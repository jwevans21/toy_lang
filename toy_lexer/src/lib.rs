use core::str::Chars;

mod token;

pub use token::{ToyToken, ToyTokenKind};

#[derive(Debug)]
pub struct ToyLexer<'src> {
    src: &'src str,
    chars: Chars<'src>,
    pos: usize,
}

impl<'src> ToyLexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            chars: src.chars(),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<ToyToken> {
        loop {
            self.skip_whitespace();

            let start = self.pos;

            let c = self.next_char()?;

            let token_kind = match c {
                '(' => ToyTokenKind::LeftParen,
                ')' => ToyTokenKind::RightParen,
                '{' => ToyTokenKind::LeftBrace,
                '}' => ToyTokenKind::RightBrace,
                '[' => ToyTokenKind::LeftBracket,
                ']' => ToyTokenKind::RightBracket,
                ',' => ToyTokenKind::Comma,
                ';' => ToyTokenKind::Semicolon,
                ':' => ToyTokenKind::Colon,

                '+' => ToyTokenKind::OperatorAdd,
                '-' => ToyTokenKind::OperatorSubtract,
                '*' => ToyTokenKind::OperatorMultiply,
                '/' => {
                    if let Some('/') = self.peek_char() {
                        self.next_char();
                        self.skip_line();
                        continue;
                    } else {
                        ToyTokenKind::OperatorDivide
                    }
                }
                '%' => ToyTokenKind::OperatorModulo,

                '=' => {
                    if let Some('=') = self.peek_char() {
                        self.next_char();
                        ToyTokenKind::OperatorEquality
                    } else {
                        ToyTokenKind::OperatorAssign
                    }
                }
                '!' => {
                    if let Some('=') = self.chars.clone().next() {
                        self.next_char();
                        ToyTokenKind::OperatorNotEqual
                    } else {
                        return None;
                    }
                }
                '>' => {
                    if let Some('=') = self.chars.clone().next() {
                        self.next_char();
                        ToyTokenKind::OperatorGreaterThanEqual
                    } else {
                        ToyTokenKind::OperatorGreaterThan
                    }
                }
                '<' => {
                    if let Some('=') = self.chars.clone().next() {
                        self.next_char();
                        ToyTokenKind::OperatorLessThanEqual
                    } else {
                        ToyTokenKind::OperatorLessThan
                    }
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    while let Some(c) = self.peek_char() {
                        if c.is_ascii_alphanumeric() || c == '_' {
                            self.next_char();
                        } else {
                            break;
                        }
                    }

                    let ident = &self.src[start..self.pos];

                    match ident {
                        "fn" => ToyTokenKind::KeywordFn,
                        "return" => ToyTokenKind::KeywordReturn,
                        "let" => ToyTokenKind::KeywordLet,
                        _ => ToyTokenKind::Identifier,
                    }
                }

                _ => return None,
            };

            return Some(ToyToken {
                kind: token_kind,
                start,
                end: self.pos,
            });
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.pos += 1;
        Some(c)
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.clone().next() {
            if c.is_whitespace() {
                self.chars.next();
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn skip_line(&mut self) {
        while let Some(c) = self.chars.clone().next() {
            if c == '\n' {
                self.chars.next();
                self.pos += 1;
                break;
            } else {
                self.chars.next();
                self.pos += 1;
            }
        }
    }
}
