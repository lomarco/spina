use driver::{CompileOutput, compile};
use err::Diag;
use lex::TokenKind;
use sess::{Options, Session, Source, Stage};

fn compile_source(text: &str, emit: Option<Stage>) -> Result<CompileOutput, Vec<Diag>> {
    let sess = Session::new(
        Options { emit, output: None },
        Source {
            name: None,
            text: text.to_owned(),
        },
    );

    compile(&sess)
}

#[test]
fn emits_token_stream_for_tokens_stage() {
    let result = compile_source("let a: i32 = 2;", Some(Stage::Tok)).unwrap();

    let CompileOutput::Tokens(stream) = result else {
        panic!("expected a token stream artifact");
    };

    let token = stream.get(0).expect("token stream is not empty");
    assert!(matches!(token.kind, TokenKind::Ident(_)));
}

#[test]
fn emits_ast_for_ast_stage() {
    let result = compile_source("fn foo(): i8 {}", Some(Stage::Ast)).unwrap();

    let CompileOutput::Ast(unit) = result else {
        panic!("expected an ast artifact");
    };

    assert_eq!(unit.items.len(), 1);
}

#[test]
fn reports_lex_error_for_invalid_source() {
    let errs = compile_source("let \u{00a7} = 1;", None).unwrap_err();

    assert_eq!(errs.len(), 1);
}

#[test]
fn reports_parse_error_for_invalid_source() {
    let errs = compile_source("fn foo( {}", Some(Stage::Ast)).unwrap_err();

    assert_eq!(errs.len(), 1);
}
