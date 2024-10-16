#![feature(let_chains)]

use core::str::Chars;

#[cfg(test)]
mod tests;
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

                '+' => {
                    if let Some(c) = self.peek_char() && c.is_ascii_digit() {
                        self.consume_integer();
                        ToyTokenKind::LiteralInteger
                    } else {
                        ToyTokenKind::OperatorAdd
                    }
                }
                '-' => {
                    if let Some(c) = self.peek_char() && c.is_ascii_digit() {
                        self.consume_integer();
                        ToyTokenKind::LiteralInteger
                    } else {
                        ToyTokenKind::OperatorSubtract
                    }
                },
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
                    if let Some('=') = self.peek_char() {
                        self.next_char();
                        ToyTokenKind::OperatorNotEqual
                    } else {
                        return None;
                    }
                }
                '>' => {
                    if let Some('=') = self.peek_char(){
                        self.next_char();
                        ToyTokenKind::OperatorGreaterThanEqual
                    } else {
                        ToyTokenKind::OperatorGreaterThan
                    }
                }
                '<' => {
                    if let Some('=') = self.peek_char() {
                        self.next_char();
                        ToyTokenKind::OperatorLessThanEqual
                    } else {
                        ToyTokenKind::OperatorLessThan
                    }
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    self.consume_identifier_or_keyword();

                    let ident = &self.src[start..self.pos];

                    match ident {
                        "fn" => ToyTokenKind::KeywordFn,
                        "return" => ToyTokenKind::KeywordReturn,
                        "let" => ToyTokenKind::KeywordLet,
                        "if" => ToyTokenKind::KeywordIf,
                        "else" => ToyTokenKind::KeywordElse,
                        "while" => ToyTokenKind::KeywordWhile,

                        "true" => ToyTokenKind::LiteralTrue,
                        "false" => ToyTokenKind::LiteralFalse,
                        _ => ToyTokenKind::Identifier,
                    }
                }

                '0'..='9' => {
                    self.consume_integer();
                    ToyTokenKind::LiteralInteger
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

    fn consume_while<F>(&mut self, mut f: F) -> usize
    where
        F: FnMut(char) -> bool,
    {
        let start = self.pos;

        while let Some(c) = self.peek_char() {
            if f(c) {
                self.next_char();
            } else {
                break;
            }
        }

        self.pos - start
    }

    fn skip_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn skip_line(&mut self) {
        self.consume_while(|c| c != '\n');
        self.next_char(); // consume '\n'
    }

    fn consume_identifier_or_keyword(&mut self) -> usize {
        self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_')
    }

    fn consume_integer(&mut self) -> usize {
        self.consume_while(|c| c.is_ascii_digit())
    }
}
