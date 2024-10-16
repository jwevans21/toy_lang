use super::*;

#[test]
fn keyword_fn() {
    let src = "fn";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::KeywordFn);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 2);
}

#[test]
fn keyword_return() {
    let src = "return";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::KeywordReturn);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 6);
}

#[test]
fn keyword_let() {
    let src = "let";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::KeywordLet);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 3);
}

#[test]
fn keyword_if() {
    let src = "if";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::KeywordIf);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 2);
}

#[test]
fn keyword_else() {
    let src = "else";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::KeywordElse);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 4);
}

#[test]
fn keyword_while() {
    let src = "while";

    let mut lexer = ToyLexer::new(src);
    let token = lexer.next_token();

    assert_eq!(token.is_some(), true);

    let token = token.unwrap();
    assert_eq!(token.kind, ToyTokenKind::KeywordWhile);
    assert_eq!(token.start, 0);
    assert_eq!(token.end, 5);
}
