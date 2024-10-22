use super::*;

#[test]
fn literal_true() {
    let src = "true";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::LiteralTrue);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 4);
}

#[test]
fn literal_false() {
    let src = "false";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::LiteralFalse);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 5);
}

#[test]
fn literal_integer_basic() {
    let src = "123";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::LiteralInteger);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 3);
}

#[test]
fn literal_integer_negative() {
    let src = "-123";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::LiteralInteger);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 4);
    assert_eq!(&src[token.start..token.end], "-123");
}

#[test]
fn literal_integer_positive() {
    let src = "+123";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::LiteralInteger);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 4);
    assert_eq!(&src[token.start..token.end], "+123");
}
