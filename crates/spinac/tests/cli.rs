use std::process::Command;

fn spinac() -> Command {
    Command::new(env!("CARGO_BIN_EXE_spinac"))
}

#[test]
fn emits_ast_for_valid_source() {
    let output = spinac()
        .args(["--emit", "ast", "-e", "fn foo(): i8 {}"])
        .output()
        .expect("failed to spawn spinac");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Unit("));
}

#[test]
fn emits_tokens_for_valid_source() {
    let output = spinac()
        .args(["--emit", "tokens", "-e", "let a = 1;"])
        .output()
        .expect("failed to spawn spinac");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("TokenStream"));
}

#[test]
fn fails_on_lex_error() {
    let output = spinac()
        .args(["--emit", "ast", "-e", "let \u{00a7} = 1;"])
        .output()
        .expect("failed to spawn spinac");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unexpected char"));
}

#[test]
fn fails_on_parse_error() {
    let output = spinac()
        .args(["--emit", "ast", "-e", "fn foo( {}"])
        .output()
        .expect("failed to spawn spinac");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Error:"));
}
