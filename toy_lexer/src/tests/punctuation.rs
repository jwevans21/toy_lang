use super::*;

#[test]
fn punctuation_left_paren() {
    let src = "(";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::LeftParen);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn punctuation_right_paren() {
    let src = ")";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::RightParen);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn punctuation_left_brace() {
    let src = "{";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::LeftBrace);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn punctuation_right_brace() {
    let src = "}";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::RightBrace);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn punctuation_left_bracket() {
    let src = "[";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::LeftBracket);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn punctuation_right_bracket() {
    let src = "]";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::RightBracket);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn punctuation_comma() {
    let src = ",";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::Comma);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn punctuation_semicolon() {
    let src = ";";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::Semicolon);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}

#[test]
fn punctuation_colon() {
    let src = ":";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::Colon);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 1);
}
