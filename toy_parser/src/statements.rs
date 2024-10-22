use toy_ast::ToyStatement;
use toy_lexer::ToyTokenKind;

use crate::ToyParser;

impl<'src> ToyParser<'src> {
    pub(crate) fn parse_statement(&mut self) -> ToyStatement<'src> {
        let token = self.lexer.peek_token().expect("Unexpected end of file");

        match token.kind {
            ToyTokenKind::LeftBrace => ToyStatement::Block {
                statements: self.parse_block(),
            },
            ToyTokenKind::KeywordLet => self.parse_variable_declaration(),
            ToyTokenKind::KeywordIf => self.parse_if_statement(),
            ToyTokenKind::KeywordWhile => self.parse_while_statement(),
            ToyTokenKind::KeywordReturn => {
                self.assert_token(ToyTokenKind::KeywordReturn);

                let value =
                    if self.expect_token(ToyTokenKind::Semicolon).is_none() {
                        Some(Box::new(self.parse_expression()))
                    } else {
                        None
                    };

                self.assert_token(ToyTokenKind::Semicolon);

                ToyStatement::Return { value }
            }
            _ => {
                let statement = ToyStatement::Expression {
                    value: self.parse_expression(),
                };
                self.assert_token(ToyTokenKind::Semicolon);
                statement
            }
        }
    }

    fn parse_block(&mut self) -> Vec<ToyStatement<'src>> {
        self.assert_token(ToyTokenKind::LeftBrace);

        let mut statements = Vec::new();

        loop {
            let next = self.lexer.peek_token().expect("Unexpected end of file");

            if next.kind == ToyTokenKind::RightBrace {
                break;
            }

            let statement = self.parse_statement();
            statements.push(statement);
        }

        self.assert_token(ToyTokenKind::RightBrace);

        statements
    }

    fn parse_variable_declaration(&mut self) -> ToyStatement<'src> {
        self.assert_token(ToyTokenKind::KeywordLet);

        let name_token = self.assert_token(ToyTokenKind::Identifier);
        let name = &self.src[name_token.range()];

        let ty = if self.expect_token(ToyTokenKind::Colon).is_some() {
            self.lexer.next_token();
            Some(self.parse_type())
        } else {
            None
        };

        self.assert_token(ToyTokenKind::OperatorAssign);

        let value = self.parse_expression();

        self.assert_token(ToyTokenKind::Semicolon);

        ToyStatement::VariableDeclaration { name, ty, value }
    }

    fn parse_if_statement(&mut self) -> ToyStatement<'src> {
        self.assert_token(ToyTokenKind::KeywordIf);

        let condition = self.parse_expression();
        let then_block = self.parse_statement();

        let else_branch =
            if self.expect_token(ToyTokenKind::KeywordElse).is_some() {
                Some(Box::new(self.parse_statement()))
            } else {
                None
            };

        ToyStatement::If {
            condition,
            then_branch: Box::new(then_block),
            else_branch,
        }
    }

    fn parse_while_statement(&mut self) -> ToyStatement<'src> {
        self.assert_token(ToyTokenKind::KeywordWhile);

        let condition = self.parse_expression();
        let body = Box::new(self.parse_statement());

        ToyStatement::While { condition, body }
    }
}
