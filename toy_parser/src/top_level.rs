use toy_ast::{ToyTopLevelExpression, ToyType};
use toy_lexer::ToyTokenKind;

use crate::ToyParser;

impl<'src> ToyParser<'src> {
    pub(crate) fn parse_top_level_expression(
        &mut self,
    ) -> Option<ToyTopLevelExpression<'src>> {
        let token = self.lexer.peek_token()?;

        match token.kind {
            ToyTokenKind::KeywordExtern => self.parse_extern(),
            ToyTokenKind::KeywordFn => self.parse_function_definition(),
            _ => None,
        }
    }

    fn parse_extern(&mut self) -> Option<ToyTopLevelExpression<'src>> {
        self.assert_token(ToyTokenKind::KeywordExtern);

        // Only functions are supported for now
        self.assert_token(ToyTokenKind::KeywordFn);

        let name_token = self.assert_token(ToyTokenKind::Identifier);
        let name = &self.src[name_token.range()];

        self.assert_token(ToyTokenKind::LeftParen);

        let parameters = self.parse_extern_function_parameters();

        self.assert_token(ToyTokenKind::RightParen);

        let return_type = self.parse_type_definition();

        Some(ToyTopLevelExpression::ExternalFunctionDeclaration {
            name,
            parameters,
            return_type,
        })
    }

    fn parse_extern_function_parameters(&mut self) -> Vec<ToyType<'src>> {
        let mut parameters = Vec::new();

        let next = self.lexer.peek_token().expect("Unexpected end of file");

        if next.kind == ToyTokenKind::RightParen {
            return parameters;
        }

        // a loop to handle multiple paramters that optionally have a name
        loop {
            let type_token =
                self.lexer.next_token().expect("Unexpected end of file");

            let ty = self.parse_type_token(&type_token);

            let ty = match ty {
                Some(ty)
                    if self.lexer.peek_token().is_some_and(|token| {
                        token.kind != ToyTokenKind::Colon
                    }) =>
                {
                    ty
                }
                Some(_)
                    if self.lexer.peek_token().is_some_and(|token| {
                        token.kind == ToyTokenKind::Colon
                    }) =>
                {
                    self.parse_type_definition()
                }
                None if self
                    .lexer
                    .peek_token()
                    .is_some_and(|token| token.kind == ToyTokenKind::Colon) =>
                {
                    self.parse_type_definition()
                }
                Some(_) | None => {
                    panic!("Expected type but got {:?}", type_token.kind)
                }
            };

            parameters.push(ty);

            let next = self.lexer.peek_token().expect("Unexpected end of file");

            if next.kind == ToyTokenKind::RightParen {
                break;
            }

            self.assert_token(ToyTokenKind::Comma);
        }

        parameters
    }

    fn parse_function_definition(
        &mut self,
    ) -> Option<ToyTopLevelExpression<'src>> {
        self.assert_token(ToyTokenKind::KeywordFn);

        let name_token = self.assert_token(ToyTokenKind::Identifier);
        let name = &self.src[name_token.range()];

        self.assert_token(ToyTokenKind::LeftParen);
        let parameters = self.parse_parameters();
        self.assert_token(ToyTokenKind::RightParen);

        let return_type = self.parse_type_definition();

        let body = self.parse_statement();

        Some(ToyTopLevelExpression::FunctionDefinition {
            name,
            parameters,
            return_type,
            body,
        })
    }
}
