use super::*;

#[test]
fn operator_add() {
    let src = "+";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorAdd);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn operator_subtract() {
    let src = "-";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorSubtract);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn operator_multiply() {
    let src = "*";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorMultiply);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn operator_divide() {
    let src = "/";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorDivide);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn operator_modulo() {
    let src = "%";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorModulo);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn operator_equality() {
    let src = "==";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorEquality);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 2);
}

#[test]
fn operator_not_equal() {
    let src = "!=";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorNotEqual);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 2);
}

#[test]
fn operator_greater_than() {
    let src = ">";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorGreaterThan);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn operator_greater_than_equal() {
    let src = ">=";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorGreaterThanEqual);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 2);
}

#[test]
fn operator_less_than() {
    let src = "<";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorLessThan);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn operator_less_than_equal() {
    let src = "<=";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorLessThanEqual);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 2);
}

#[test]
fn operator_assign() {
    let src = "=";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::OperatorAssign);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}
