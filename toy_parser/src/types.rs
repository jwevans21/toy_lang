use toy_ast::ToyType;
use toy_lexer::{ToyToken, ToyTokenKind};

use crate::ToyParser;

impl<'src> ToyParser<'src> {
    pub(crate) fn parse_type_definition(&mut self) -> ToyType<'src> {
        self.assert_token(ToyTokenKind::Colon);

        self.parse_type()
    }

    pub(crate) fn parse_type(&mut self) -> ToyType<'src> {
        let token = self.lexer.next_token().expect("Unexpected end of file");

        match self.parse_type_token(&token) {
            Some(ty) => ty,
            None => panic!("Expected type but got {:?}", token.kind),
        }
    }

    pub(crate) fn parse_type_token(
        &self,
        token: &ToyToken,
    ) -> Option<ToyType<'src>> {
        if token.kind == ToyTokenKind::Identifier {
            let ident = &self.src[token.range()];

            match ident {
                "int" => Some(ToyType::Int),
                "bool" => Some(ToyType::Bool),
                "void" => Some(ToyType::Void),
                _ => None,
            }
        } else {
            None
        }
    }
}
