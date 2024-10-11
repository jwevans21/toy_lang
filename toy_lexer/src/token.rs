#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToyToken {
    pub kind: ToyTokenKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
