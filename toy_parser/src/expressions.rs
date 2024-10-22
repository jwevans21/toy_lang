use toy_ast::{ToyBinaryOperator, ToyExpression, ToyLiteral};
use toy_lexer::{ToyToken, ToyTokenKind};

use crate::ToyParser;

impl<'src> ToyParser<'src> {
    pub(crate) fn parse_expression(&mut self) -> ToyExpression<'src> {
        let lhs = self.parse_primary();

        self.parse_rhs(0, lhs)
    }

    const fn get_operator_precedence(kind: ToyTokenKind) -> Option<u8> {
        match kind {
            ToyTokenKind::OperatorAdd | ToyTokenKind::OperatorSubtract => {
                Some(1)
            }
            ToyTokenKind::OperatorMultiply
            | ToyTokenKind::OperatorDivide
            | ToyTokenKind::OperatorModulo => Some(2),
            _ => None,
        }
    }

    const fn token_kind_to_binary_operator(
        kind: ToyTokenKind,
    ) -> Option<ToyBinaryOperator> {
        match kind {
            ToyTokenKind::OperatorAdd => Some(ToyBinaryOperator::Add),
            ToyTokenKind::OperatorSubtract => Some(ToyBinaryOperator::Subtract),
            ToyTokenKind::OperatorMultiply => Some(ToyBinaryOperator::Multiply),
            ToyTokenKind::OperatorDivide => Some(ToyBinaryOperator::Divide),
            ToyTokenKind::OperatorModulo => Some(ToyBinaryOperator::Modulo),
            _ => None,
        }
    }

    fn parse_primary(&mut self) -> ToyExpression<'src> {
        let token = self.lexer.next_token().expect("Unexpected end of file");

        match token.kind {
            ToyTokenKind::LiteralTrue => ToyExpression::Literal {
                value: ToyLiteral::Bool(true),
            },
            ToyTokenKind::LiteralFalse => ToyExpression::Literal {
                value: ToyLiteral::Bool(false),
            },
            ToyTokenKind::LiteralInteger => {
                let value = self.src[token.range()]
                    .parse()
                    .expect("Invalid integer literal");
                ToyExpression::Literal {
                    value: ToyLiteral::Int(value),
                }
            }
            ToyTokenKind::Identifier => {
                let name = &self.src[token.range()];

                let next = self.lexer.peek_token();

                match next {
                    Some(ToyToken {
                        kind: ToyTokenKind::LeftParen,
                        ..
                    }) => self.parse_call_expression(name),
                    _ => ToyExpression::Identifier { name },
                }
            }
            ToyTokenKind::LeftParen => {
                let expression = self.parse_expression();
                self.assert_token(ToyTokenKind::RightParen);
                expression
            }
            _ => panic!(
                "Expected a literal or identifier but got {:?}",
                token.kind
            ),
        }
    }

    fn parse_call_expression(
        &mut self,
        name: &'src str,
    ) -> ToyExpression<'src> {
        self.assert_token(ToyTokenKind::LeftParen);

        let mut arguments = Vec::new();

        let next = self.lexer.peek_token().expect("Unexpected end of file");

        if next.kind == ToyTokenKind::RightParen {
            self.lexer.next_token();
            return ToyExpression::Call {
                function: name,
                arguments,
            };
        }

        loop {
            let argument = self.parse_expression();
            arguments.push(argument);

            let next = self.lexer.peek_token().expect("Unexpected end of file");

            if next.kind == ToyTokenKind::RightParen {
                break;
            }

            self.assert_token(ToyTokenKind::Comma);
        }

        self.assert_token(ToyTokenKind::RightParen);

        ToyExpression::Call {
            function: name,
            arguments,
        }
    }

    fn parse_rhs(
        &mut self,
        prec: u8,
        lhs: ToyExpression<'src>,
    ) -> ToyExpression<'src> {
        let mut lhs = lhs;

        loop {
            let token =
                self.lexer.peek_token().expect("Unexpected end of file");

            let next_prec = Self::get_operator_precedence(token.kind);

            if !next_prec.is_some_and(|next_prec| next_prec >= prec) {
                return lhs;
            }

            // let next_prec = next_prec.unwrap();

            let token = self.lexer.next_token().unwrap();

            let mut rhs = self.parse_primary();

            let next = self.lexer.peek_token().expect("Unexpected end of file");

            let next_prec = Self::get_operator_precedence(next.kind);

            if next_prec.is_some_and(|next_prec| next_prec > next_prec) {
                let next_prec = next_prec.unwrap();

                self.lexer.next_token();
                rhs = self.parse_rhs(next_prec, rhs);
            }

            lhs = ToyExpression::Binary {
                lhs: Box::new(lhs),
                operator: Self::token_kind_to_binary_operator(token.kind)
                    .expect("Invalid binary operator"),
                rhs: Box::new(rhs),
            };
        }
    }
}
