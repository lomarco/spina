use super::*;
use crate::ast::*;
use crate::{new_parser_from_str, unwrap_or_emit_fatal};
use lex::{TokenCursor, TokenKind, flex};

fn parser(s: &str) -> Parser {
    unwrap_or_emit_fatal(new_parser_from_str(s))
}

fn unit(s: &str) -> Unit {
    parser(s).parse_unit().unwrap()
}

fn stmt(s: &str) -> Stmt {
    parser(s).parse_stmt().unwrap()
}

fn expr(s: &str) -> Expr {
    parser(s).parse_expr().unwrap()
}

#[test]
fn empty_unit() {
    assert_eq!(unit("").0.len(), 0)
}

#[test]
fn fn_item() {
    let unit = unit("fn foo(): i8 {}");

    let ItemKind::Fn(fn_item) = &unit.0[0].kind else {
        panic!();
    };

    assert_eq!(fn_item.ident.get_name_str(), "foo");
    assert_eq!(*fn_item.decl.ty.get_kind(), TyKind::Int(IntTy::I8));
    assert!(fn_item.body.stmts.is_empty());
}

#[test]
fn let_stmt() {
    let stmt = stmt("let a: i32 = 2;");

    let StmtKind::Let(ident, ty, expr, _) = stmt.kind else {
        panic!();
    };

    assert_eq!(ident.get_name_str(), "a");
    assert_eq!(*ty.get_kind(), TyKind::Int(IntTy::I32));
    assert!(matches!(expr.kind, ExprKind::Lit(Lit::Integer(2))));
}

#[test]
fn parse_divs_expr() {
    let expr = expr("2 / 3");

    let ExprKind::Binary(op, lhs, rhs) = expr.kind else {
        panic!();
    };

    assert!(matches!(op.kind, BinOpKind::Div));
    assert!(matches!(lhs.kind, ExprKind::Lit(Lit::Integer(2))));
    assert!(matches!(rhs.kind, ExprKind::Lit(Lit::Integer(3))));
}

#[test]
fn parse_plus_expr() {
    let expr = expr("2 + 3");

    let ExprKind::Binary(op, lhs, rhs) = expr.kind else {
        panic!();
    };

    assert!(matches!(op.kind, BinOpKind::Add));
    assert!(matches!(lhs.kind, ExprKind::Lit(Lit::Integer(2))));
    assert!(matches!(rhs.kind, ExprKind::Lit(Lit::Integer(3))));
}

#[test]
fn parse_mul_expr() {
    let expr = expr("2 * 3");

    let ExprKind::Binary(op, lhs, rhs) = expr.kind else {
        panic!();
    };

    assert!(matches!(op.kind, BinOpKind::Mul));
    assert!(matches!(lhs.kind, ExprKind::Lit(Lit::Integer(2))));
    assert!(matches!(rhs.kind, ExprKind::Lit(Lit::Integer(3))));
}

#[test]
fn parse_min_expr() {
    let expr = expr("2 - 3");

    let ExprKind::Binary(op, lhs, rhs) = expr.kind else {
        panic!();
    };

    assert!(matches!(op.kind, BinOpKind::Sub));
    assert!(matches!(lhs.kind, ExprKind::Lit(Lit::Integer(2))));
    assert!(matches!(rhs.kind, ExprKind::Lit(Lit::Integer(3))));
}

#[test]
fn parses_parens_expr() {
    let expr = expr("2 - 3 / (3 - 2)");

    let ExprKind::Binary(op, lhs, rhs) = expr.kind else {
        panic!("expected binary expression, got {expr:?}");
    };

    assert!(matches!(op.kind, BinOpKind::Sub));
    assert!(matches!(lhs.kind, ExprKind::Lit(_)));

    let ExprKind::Binary(div_op, div_lhs, div_rhs) = rhs.kind else {
        panic!("expected division on rhs, got {rhs:?}");
    };

    assert!(matches!(div_op.kind, BinOpKind::Div));
    assert!(matches!(div_lhs.kind, ExprKind::Lit(_)));

    let ExprKind::Binary(paren_op, paren_lhs, paren_rhs) = div_rhs.kind else {
        panic!("expected parenthesized subtraction, got {div_rhs:?}");
    };

    assert!(matches!(paren_op.kind, BinOpKind::Sub));
    assert!(matches!(paren_lhs.kind, ExprKind::Lit(_)));
    assert!(matches!(paren_rhs.kind, ExprKind::Lit(_)));
}

fn token_spans(s: &str) -> Vec<Span> {
    let stream = flex(s).unwrap();
    let mut cursor = TokenCursor::new(stream);

    let mut spans = Vec::new();

    loop {
        let token = cursor.next_and_bump();
        spans.push(token.span);

        if token.kind == TokenKind::Eof {
            break;
        }
    }

    spans
}

#[test]
fn ident_expr_has_ident_token_span() {
    let spans = token_spans("foo");
    let expr = expr("foo");

    let ExprKind::Ident(ident) = expr.kind else {
        panic!("expected ident expression, got {expr:?}");
    };

    assert_eq!(ident.get_span(), spans[0]);
    assert_eq!(expr.span, spans[0]);
}

#[test]
fn int_literal_expr_has_literal_token_span() {
    let spans = token_spans("42");
    let expr = expr("42");

    assert_eq!(expr.span, spans[0]);
}

#[test]
fn let_ident_has_ident_token_span() {
    let src = "let a: i32 = 2;";
    let spans = token_spans(src);

    let stmt = stmt(src);

    let StmtKind::Let(ident, _, _, _) = stmt.kind else {
        panic!("expected let statement, got {stmt:?}");
    };

    assert_eq!(ident.get_span(), spans[1]);
}

#[test]
fn let_stmt_has_let_keyword_span() {
    let src = "let a: i32 = 2;";
    let spans = token_spans(src);

    let stmt = stmt(src);

    assert_eq!(stmt.span, spans[0]);
}

#[test]
fn binary_expr_has_operator_or_operand_span_not_eof_span() {
    let src = "1 + 2";
    let spans = token_spans(src);

    let expr = expr(src);

    assert_ne!(expr.span, *spans.last().unwrap());

    assert_eq!(expr.span, spans[1]);
}

#[test]
fn const_item_has_const_keyword_span() {
    let src = "const X: i32 = 1;";
    let spans = token_spans(src);

    let unit = unit(src);

    assert_eq!(unit.0[0].span, spans[0]);
}

#[test]
fn call_stmt() {
    let src = "foo(a, 2);";

    let stmt = stmt(src);

    let StmtKind::Semi(e) = stmt.kind else {
        panic!();
    };

    let ExprKind::Call(ident, exprs) = e.kind else {
        panic!();
    };

    assert_eq!(ident.get_name_str(), "foo");
    assert_eq!(exprs.len(), 2);

    let ExprKind::Ident(first) = exprs[0].kind else {
        panic!();
    };

    assert_eq!(first.get_name_str(), "a");

    let ExprKind::Lit(Lit::Integer(second)) = exprs[1].kind else {
        panic!();
    };

    assert_eq!(second, 2);
}

#[test]
fn one_arg_call_stmt() {
    let src = "foo(a);";

    let stmt = stmt(src);

    let StmtKind::Semi(e) = stmt.kind else {
        panic!();
    };

    let ExprKind::Call(ident, exprs) = e.kind else {
        panic!();
    };

    assert_eq!(ident.get_name_str(), "foo");
    assert_eq!(exprs.len(), 1);

    let ExprKind::Ident(first) = exprs[0].kind else {
        panic!();
    };

    assert_eq!(first.get_name_str(), "a");
}

#[test]
fn empty_call_stmt() {
    let src = "foo();";

    let stmt = stmt(src);

    let StmtKind::Semi(e) = stmt.kind else {
        panic!();
    };

    let ExprKind::Call(ident, exprs) = e.kind else {
        panic!();
    };

    assert_eq!(ident.get_name_str(), "foo");
    assert_eq!(exprs.len(), 0);
}

#[test]
fn multy_args_expr_call() {
    let src = "foo(a, b, c)";

    let expr = expr(src);

    let ExprKind::Call(ident, exprs) = expr.kind else {
        panic!("expected call expression");
    };

    assert_eq!(ident.get_name_str(), "foo");
    assert_eq!(exprs.len(), 3);

    for (expr, expected_name) in exprs.iter().zip(["a", "b", "c"]) {
        let ExprKind::Ident(ident) = expr.kind else {
            panic!("expected identifier argument");
        };

        assert_eq!(ident.get_name_str(), expected_name);
    }
}

#[test]
fn empty_args_expr_call() {
    let src = "foo()";

    let expr = expr(src);

    let ExprKind::Call(ident, exprs) = expr.kind else {
        panic!("expected call expression");
    };

    assert_eq!(ident.get_name_str(), "foo");
    assert_eq!(exprs.len(), 0);
}

#[test]
fn math_expr_call() {
    let src = "(foo() + 2) / s";

    let expr = expr(src);

    let ExprKind::Binary(div_op, div_lhs, div_rhs) = expr.kind else {
        panic!("expected division expression");
    };

    assert!(matches!(div_op.kind, BinOpKind::Div));

    let ExprKind::Binary(add_op, add_lhs, add_rhs) = div_lhs.kind else {
        panic!("expected addition expression");
    };

    assert!(matches!(add_op.kind, BinOpKind::Add));

    let ExprKind::Call(ident, args) = add_lhs.kind else {
        panic!("expected call expression");
    };

    assert_eq!(ident.get_name_str(), "foo");
    assert_eq!(args.len(), 0);

    let ExprKind::Lit(Lit::Integer(value)) = add_rhs.kind else {
        panic!("expected integer literal");
    };

    assert_eq!(value, 2);

    let ExprKind::Ident(ident) = div_rhs.kind else {
        panic!("expected identifier");
    };

    assert_eq!(ident.get_name_str(), "s");
}
