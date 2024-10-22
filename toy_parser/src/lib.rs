use toy_ast::{ToyTopLevelExpression, ToyType};
use toy_lexer::{ToyLexer, ToyToken, ToyTokenKind};

mod expressions;
mod statements;
mod top_level;
mod types;

#[derive(Debug)]
pub struct ToyParser<'src> {
    src: &'src str,
    lexer: ToyLexer<'src>,
}

impl<'src> ToyParser<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            lexer: ToyLexer::new(src),
        }
    }

    fn expect_token(&mut self, expected: ToyTokenKind) -> Option<ToyToken> {
        let token = self.lexer.peek_token()?;

        if token.kind == expected {
            Some(token)
        } else {
            None
        }
    }

    fn assert_token(&mut self, expected: ToyTokenKind) -> ToyToken {
        match self.expect_token(expected) {
            Some(_) => self.lexer.next_token().unwrap(),
            None => {
                let token =
                    self.lexer.next_token().expect("Unexpected end of file");
                panic!(
                    "Expected token kind {:?} but got {:?} `{}`",
                    expected,
                    token.kind,
                    &self.src[token.range()]
                );
            }
        }
    }

    pub fn parse(&mut self) -> Vec<ToyTopLevelExpression<'src>> {
        let mut lex = self.lexer.clone();
        while let Some(token) = lex.next_token() {
            println!("{:?} `{}`", token, &self.src[token.range()]);
        }

        let mut result = Vec::new();

        while let Some(top_level_expression) = self.parse_top_level_expression()
        {
            result.push(top_level_expression);
        }

        result
    }

    fn parse_parameters(&mut self) -> Vec<(&'src str, ToyType<'src>)> {
        let mut parameters = Vec::new();

        let next = self.lexer.peek_token().expect("Unexpected end of file");

        if next.kind == ToyTokenKind::RightParen {
            return parameters;
        }

        loop {
            let name_token = self.assert_token(ToyTokenKind::Identifier);
            let name = &self.src[name_token.range()];

            self.assert_token(ToyTokenKind::Colon);

            let ty = self.parse_type();

            parameters.push((name, ty));

            let next = self.lexer.peek_token().expect("Unexpected end of file");

            match next.kind {
                ToyTokenKind::Comma => {
                    self.lexer.next_token();
                }
                ToyTokenKind::RightParen => {
                    break;
                }
                _ => panic!("Expected `,` or `)` but got {:?}", next.kind),
            }
        }

        parameters
    }
}
