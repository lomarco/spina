use super::*;
use crate::resolve::Resolution;
use ast::{Expr, ExprKind, ItemKind, NodeId, Stmt, StmtKind, Unit};

fn parse(src: &str) -> Unit {
    let stream = lex::flex(src).unwrap();
    parser::parse(stream).unwrap()
}

fn resolve_ok(src: &str) -> (Unit, Resolved) {
    let unit = parse(src);
    let resolved = resolve(&unit).unwrap();
    (unit, resolved)
}

fn assert_resolves_to(resolved: &Resolved, expr: &Expr, expected_node: NodeId) {
    match resolved.resolution(expr.id) {
        Resolution::Local(def) | Resolution::Item(def) => {
            assert_eq!(resolved.def(def).node.index(), expected_node.index())
        }
        other => panic!("expected a resolved reference, got {other:?}"),
    }
}

fn call_callee(expr: &Expr) -> &Expr {
    let ExprKind::Call(callee, _) = &expr.kind else {
        panic!("expected a call expression, got {expr:?}");
    };
    callee
}

fn call_args(expr: &Expr) -> &[Expr] {
    let ExprKind::Call(_, args) = &expr.kind else {
        panic!("expected a call expression, got {expr:?}");
    };
    args
}

fn stmt_expr(stmt: &Stmt) -> &Expr {
    match &stmt.kind {
        StmtKind::Semi(expr) | StmtKind::Expr(expr) => expr,
        other => panic!("expected an expression statement, got {other:?}"),
    }
}

#[test]
fn resolves_parameters() {
    let src = "fn main(x: i32): i32 { foo(x); } fn foo(y: i32): i32 {}";
    let (unit, resolved) = resolve_ok(src);

    let ItemKind::Fn(main) = &unit.items[0].kind else {
        panic!("expected main");
    };

    let call = stmt_expr(&main.body.stmts[0]);
    let callee = call_callee(call);
    let ExprKind::Ident(ident) = &callee.kind else {
        panic!("expected ident callee");
    };
    assert_eq!(ident.get_name_str(), "foo");
    assert_resolves_to(&resolved, callee, unit.items[1].id);

    assert_resolves_to(&resolved, &call_args(call)[0], main.decl.params[0].id);
}

#[test]
fn resolves_forward_function_call() {
    let src = "fn main(): i32 { foo(); } fn foo(): i32 {}";
    let (unit, resolved) = resolve_ok(src);

    let ItemKind::Fn(main) = &unit.items[0].kind else {
        panic!("expected main");
    };

    let callee = call_callee(stmt_expr(&main.body.stmts[0]));
    assert_resolves_to(&resolved, callee, unit.items[1].id);
}

#[test]
fn resolves_recursive_call() {
    let src = "fn main(): i32 { main(); }";
    let (unit, resolved) = resolve_ok(src);

    let ItemKind::Fn(main) = &unit.items[0].kind else {
        panic!("expected main");
    };

    let callee = call_callee(stmt_expr(&main.body.stmts[0]));
    assert_resolves_to(&resolved, callee, unit.items[0].id);
}

#[test]
fn resolves_local_reference() {
    let src = "fn main(): i32 { let a: i32 = 1; foo(a); } fn foo(y: i32): i32 {}";
    let (unit, resolved) = resolve_ok(src);

    let ItemKind::Fn(main) = &unit.items[0].kind else {
        panic!("expected main");
    };
    let StmtKind::Let(..) = &main.body.stmts[0].kind else {
        panic!("expected a let statement");
    };

    let call = stmt_expr(&main.body.stmts[1]);
    assert_resolves_to(&resolved, &call_args(call)[0], main.body.stmts[0].id);
}

#[test]
fn later_let_shadows_earlier_let() {
    let src = "fn main(): i32 { let a: i32 = 1; let a: i32 = 2; foo(a); } fn foo(y: i32): i32 {}";
    let (unit, resolved) = resolve_ok(src);

    let ItemKind::Fn(main) = &unit.items[0].kind else {
        panic!("expected main");
    };

    let call = stmt_expr(&main.body.stmts[2]);
    assert_resolves_to(&resolved, &call_args(call)[0], main.body.stmts[1].id);
}

#[test]
fn reports_undefined_name() {
    let src = "fn main(): i32 { missing(); }";
    let unit = parse(src);

    let errs = resolve(&unit).unwrap_err();
    assert_eq!(errs.len(), 1);
}

#[test]
fn reports_duplicate_items() {
    let src = "fn main(): i32 {} fn main(): i32 {}";
    let unit = parse(src);

    let errs = resolve(&unit).unwrap_err();
    assert_eq!(errs.len(), 1);
}

#[test]
fn reports_duplicate_parameters() {
    let src = "fn main(x: i32, x: i32): i32 {}";
    let unit = parse(src);

    let errs = resolve(&unit).unwrap_err();
    assert_eq!(errs.len(), 1);
}
