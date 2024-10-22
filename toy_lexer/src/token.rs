use core::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct ToyToken {
    pub kind: ToyTokenKind,
    pub start: usize,
    pub end: usize,
}

impl ToyToken {
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ToyTokenKind {
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `,`
    Comma,
    /// `;`
    Semicolon,
    /// `:`
    Colon,

    /// `+`
    OperatorAdd,
    /// `-`
    OperatorSubtract,
    /// `*`
    OperatorMultiply,
    /// `/`
    OperatorDivide,
    /// `%`
    OperatorModulo,

    /// `==`
    OperatorEquality,
    /// `!=`
    OperatorNotEqual,
    /// `>`
    OperatorGreaterThan,
    /// `>=`
    OperatorGreaterThanEqual,
    /// `<`
    OperatorLessThan,
    /// `<=`
    OperatorLessThanEqual,

    /// `=`
    OperatorAssign,

    /// `extern`
    KeywordExtern,
    /// `fn`
    KeywordFn,
    /// `return`
    KeywordReturn,
    /// `let`
    KeywordLet,
    /// `if`
    KeywordIf,
    /// `else`
    KeywordElse,
    /// `while`
    KeywordWhile,

    /// `true`
    LiteralTrue,
    /// `false`
    LiteralFalse,
    /// `[+-]?[0-9]+`
    LiteralInteger,
    /// `[a-zA-Z_][a-zA-Z0-9_]*`
    Identifier,
}
