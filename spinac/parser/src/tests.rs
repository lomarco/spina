use crate::{Parser, Unit, Stmt, Expr, Ty, unwrap_or_emit_fatal, new_parser_from_str};

fn str_to_parser(s: &str) -> Parser {
    unwrap_or_emit_fatal(new_parser_from_str(s))
}

fn str_to_unit(s: &str) -> Unit {
    str_to_parser(s).parse_unit().unwrap()
}

fn str_to_stmt(s: &str) -> Stmt {
    str_to_parser(s).parse_stmt().unwrap()
}

fn str_to_expr(s: &str) -> Expr {
    str_to_parser(s).parse_expr().unwrap()
}

fn str_to_ty(s: &str) -> Ty {
    str_to_parser(s).parse_ty().unwrap()
}
