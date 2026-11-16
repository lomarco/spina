use super::*;

fn token_kinds(s: &str) -> Vec<TokenKind> {
    let stream = flex(s).expect("failed to lex source");
    let mut cursor = TokenCursor::new(stream);

    let mut kinds = Vec::new();

    loop {
        let token = cursor.next_and_bump();
        kinds.push(token.kind);

        if token.kind == TokenKind::Eof {
            break;
        }
    }

    kinds
}

#[test]
fn empty_source_yields_only_eof_token() {
    assert_eq!(token_kinds(""), vec![TokenKind::Eof]);
}

#[test]
fn int_literal_token() {
    assert_eq!(
        token_kinds("42"),
        vec![TokenKind::IntLiteral(Lit::Integer(42)), TokenKind::Eof]
    );
}

#[test]
fn keywords_are_lexed_as_idents() {
    let kinds = token_kinds("let");

    assert!(matches!(kinds[0], TokenKind::Ident(_)));
}

#[test]
fn skips_whitespace_between_tokens() {
    let kinds = token_kinds("  let   a\t=\t1 ;");

    assert!(matches!(kinds[0], TokenKind::Ident(_)));
    assert!(matches!(kinds[1], TokenKind::Ident(_)));
    assert_eq!(kinds[2], TokenKind::Eq);
    assert_eq!(kinds[3], TokenKind::IntLiteral(Lit::Integer(1)));
    assert_eq!(kinds[4], TokenKind::Semi);
}

#[test]
fn string_literal_token() {
    let kinds = token_kinds(r#""hello""#);

    assert!(matches!(kinds[0], TokenKind::StringLiteral(_)));
}

#[test]
fn reports_unexpected_char_as_error() {
    let errs = flex("let \u{00a7} = 1;").unwrap_err();

    assert_eq!(errs.len(), 1);
}
