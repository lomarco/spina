use common::{Ident, IntTy, Lit, Span, Ty, TyKind, UintTy, kw, ty};
use err::{Diag, PResult, raise};
use lex::{Token, TokenCursor, TokenKind, TokenStream, flex};
use sess::{Input, Session};
use std::{fs::read_to_string, io::ErrorKind, mem::replace, path::Path};

#[derive(Debug)]
pub struct Unit(Vec<Item>);

#[derive(Debug)]
struct Param {
    ty: Ty,
    ident: Ident,
}

#[derive(Debug)]
struct FnDecl {
    params: Vec<Param>,
    ty: Ty,
}

#[derive(Debug)]
struct ForLoop {
    ident: Box<Ident>,
    iter: Box<Expr>,
    body: Box<Block>,
}

#[derive(Debug)]
struct Block {
    stmts: Vec<Stmt>,
    span: Span,
}

#[derive(Debug)]
enum BinOpKind {
    /// The `+` operator (addition)
    Add,
    /// The `-` operator (subtraction)
    Sub,
    /// The `*` operator (multiplication)
    Mul,
    /// The `/` operator (division)
    Div,
    /// The `%` operator (modulus)
    Rem,
    /// The `&&` operator (logical and)
    And,
    /// The `||` operator (logical or)
    Or,
    /// The `^` operator (bitwise xor)
    BitXor,
    /// The `&` operator (bitwise and)
    BitAnd,
    /// The `|` operator (bitwise or)
    BitOr,
    /// The `<<` operator (shift left)
    Shl,
    /// The `>>` operator (shift right)
    Shr,
    /// The `==` operator (equality)
    Eq,
    /// The `<` operator (less than)
    Lt,
    /// The `<=` operator (less than or equal to)
    Le,
    /// The `!=` operator (not equal to)
    Ne,
    /// The `>=` operator (greater than or equal to)
    Ge,
    /// The `>` operator (greater than)
    Gt,
}

#[derive(Debug)]
struct BinOp {
    kind: BinOpKind,
    span: Span,
}

impl BinOp {
    fn new(kind: BinOpKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
enum UnOp {
    /// The `*` operator for dereferencing
    Deref,
    /// The `!` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
}

#[derive(Debug)]
struct Expr {
    kind: ExprKind,
    span: Span,
}

impl Expr {
    fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
enum StmtKind {
    Let(Box<Ident>, Box<Ty>, Box<Expr>, Span),
    Semi(Box<Expr>),
    Expr(Box<Expr>),
    Empty,
}

#[derive(Debug)]
struct Stmt {
    kind: StmtKind,
    span: Span,
}

#[derive(Debug)]
enum ExprKind {
    Array(Vec<Box<Expr>>),
    Call(Box<Expr>, Vec<Box<Expr>>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    If(Box<Expr>, Box<Block>, Option<Box<Expr>>),
    While(Box<Expr>, Box<Block>),
    Index(Box<Expr>, Box<Expr>, Span),
    Break(Option<Box<Expr>>),
    Ret(Option<Box<Expr>>),
    Continue(), // FIXME: Add Label

    Lit(Lit),
    Ident(Ident),
    ForLoop(Box<ForLoop>),
    Loop(Box<Block>, Span),
}

fn unwrap_or_emit_fatal<T>(expr: Result<T, Vec<Diag>>) -> T {
    expr.unwrap_or_else(|errs| {
        for err in errs {
            err.emit();
        }
        raise()
    })
}

fn new_parser_from_file(path: &Path) -> Result<Parser, Vec<Diag>> {
    let cont = read_to_string(path).unwrap_or_else(|e| {
        // TODO: Maybe delete unwrap_or_else
        let msg = match e.kind() {
            ErrorKind::NotFound => format!("couldn't find file `{}`", path.display()),
            ErrorKind::PermissionDenied => {
                format!("permission denied when opening file `{}`", path.display())
            }
            ErrorKind::IsADirectory => format!("`{}` is a directory", path.display()),
            _ => format!("couldn't read `{}`: {}", path.display(), e),
        };

        Diag::fatal(msg)
    });

    let stream = flex(&cont)?;

    let parser = Parser::new(stream);
    Ok(parser)
}

fn new_parser_from_str(str: &str) -> Result<Parser, Vec<Diag>> {
    Ok(Parser::new(flex(str)?))
}

pub fn parse(sess: &Session) -> Unit {
    // TODO: Add new_parser_from_source_str
    unwrap_or_emit_fatal(match &sess.input {
        Input::File(file) => new_parser_from_file(file),
        Input::Str(str) => new_parser_from_str(str),
    })
    .parse_unit()
    .unwrap_or_else(|parse_error| {
        parse_error.emit();
        raise()
    })
}

struct Parser {
    token: Token,
    prev_token: Token,
    token_cursor: TokenCursor,
}

impl Parser {
    fn new(stream: TokenStream) -> Self {
        let mut parser = Parser {
            token: Token::dummy(),
            prev_token: Token::dummy(),
            token_cursor: TokenCursor::new(stream),
        };

        parser.bump();

        parser
    }

    fn parse_unit(&mut self) -> PResult<Unit> {
        Ok(Unit(self.parse_items()?))
    }

    fn parse_items(&mut self) -> PResult<Vec<Item>> {
        let mut items = Vec::with_capacity(128);

        loop {
            let Some(item) = self.parse_item()? else {
                break Ok(items);
            };
            items.push(item);
        }
    }

    fn parse_item(&mut self) -> PResult<Option<Item>> {
        let span = self.token.span;
        let kind = self.parse_item_kind()?;
        Ok(kind.map(|kind| Item { kind, span }))
    }

    fn parse_item_kind(&mut self) -> PResult<Option<ItemKind>> {
        if self.is_fn() {
            let (ident, decl, body) = self.parse_fn()?;
            Ok(Some(ItemKind::Fn(Box::new(Fn { ident, decl, body }))))
        } else if self.is_const() {
            let (ident, ty, expr) = self.parse_const()?;
            Ok(Some(ItemKind::Const(
                Box::new(ident),
                Box::new(ty),
                Box::new(expr),
                self.token.span,
            )))
        } else if self.is_eof() {
            Ok(None)
        } else {
            Err(Diag::err(
                format!("Expected item, found {:?}", self.token.kind),
                self.token.span,
            ))
        }
    }

    fn is_fn(&self) -> bool {
        self.token.is_keyword(kw::Fn)
    }

    fn is_const(&self) -> bool {
        self.token.is_keyword(kw::Const)
    }

    fn is_eof(&self) -> bool {
        self.check(TokenKind::Eof)
    }

    fn parse_fn(&mut self) -> PResult<(Ident, FnDecl, Box<Block>)> {
        self.bump();
        let ident = self.parse_ident()?;
        let decl = self.parse_fn_decl()?;
        let body = self.parse_fn_body()?;
        Ok((ident, decl, body))
    }

    fn parse_const(&mut self) -> PResult<(Ident, Ty, Expr)> {
        self.bump();

        let ident = self.parse_ident()?;
        let ty = self.parse_ty_annot()?;

        self.bump();
        let expr = self.parse_expr()?;

        if !self.check(TokenKind::Semi) {
            return Err(Diag::err(
                format!("Expected Semi, found {:#?}", self.token.kind),
                self.token.span,
            ));
        }
        self.bump();

        Ok((ident, ty, expr))
    }

    fn parse_ty_annot(&mut self) -> PResult<Ty> {
        if !self.check(TokenKind::Colon) {
            return Err(Diag::err(
                format!("Expected Colon, found {:#?}", self.token.kind).to_string(),
                self.token.span,
            ));
        }
        self.bump();
        self.parse_ty()
    }

    fn parse_stmt_kind(&mut self) -> PResult<StmtKind> {
        if self.token.is_keyword(kw::Let) {
            self.parse_let()
        } else {
            Err(Diag::err(
                format!("Expected statement, found {:#?}", self.token.kind).to_string(),
                self.token.span,
            ))
        }
    }

    fn parse_stmt(&mut self) -> PResult<Stmt> {
        let kind = self.parse_stmt_kind()?;
        let span = self.token.span;
        Ok(Stmt { kind, span })
    }

    fn parse_fn_body(&mut self) -> PResult<Box<Block>> {
        if !self.eat(TokenKind::OpenBrace) {
            return Err(Diag::err(
                "Missing fn OpenBrace".to_string(),
                self.token.span,
            ));
        }

        let mut stmts: Vec<Stmt> = Vec::with_capacity(1024);

        while self.token.kind != TokenKind::Eof
            && self.token.kind != TokenKind::CloseBrace
            && self.token.kind != TokenKind::Semi
        {
            stmts.push(self.parse_stmt()?);
        }

        if !self.eat(TokenKind::CloseBrace) {
            return Err(Diag::err(
                "Missing fn CloseBrace".to_string(),
                self.token.span,
            ));
        }

        Ok(Box::new(Block {
            stmts,
            span: self.token.span,
        }))
    }

    fn parse_expr(&mut self) -> PResult<Expr> {
        let mut lhs = self.parse_term()?;

        loop {
            let op = match self.token.kind {
                TokenKind::Plus => BinOp::new(BinOpKind::Add, self.token.span),
                TokenKind::Minus => BinOp::new(BinOpKind::Sub, self.token.span),
                _ => break,
            };

            self.bump();

            let rhs = self.parse_term()?;

            lhs = Expr::new(
                ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)),
                self.token.span,
            );
        }

        Ok(lhs)
    }

    fn parse_term(&mut self) -> PResult<Expr> {
        let mut lhs = self.parse_factor()?;

        loop {
            let op = match self.token.kind {
                TokenKind::Star => BinOp::new(BinOpKind::Mul, self.token.span),
                TokenKind::Slash => BinOp::new(BinOpKind::Div, self.token.span),
                _ => break,
            };

            self.bump();

            let rhs = self.parse_factor()?;

            lhs = Expr::new(
                ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)),
                self.token.span,
            );
        }

        Ok(lhs)
    }

    fn parse_factor(&mut self) -> PResult<Expr> {
        match self.token.kind {
            TokenKind::OpenParen => {
                self.bump();
                let node = self.parse_expr();
                // TODO: Add self.bump() to parse_expr()
                if !self.eat(TokenKind::CloseParen) {
                    return Err(Diag::err(
                        "Missing fn CloseParen".to_string(),
                        self.token.span,
                    ));
                }
                node
            }

            TokenKind::Ident(name) => {
                let span = self.token.span;

                self.bump();
                Ok(Expr::new(ExprKind::Ident(Ident::new(name, span)), span))
            }

            TokenKind::IntLiteral(ilit) => {
                let span = self.token.span;

                self.bump();

                Ok(Expr::new(ExprKind::Lit(ilit), span))
            }

            TokenKind::FloatLiteral(flit) => {
                let span = self.token.span;

                self.bump();

                Ok(Expr::new(ExprKind::Lit(flit), span))
            }

            _ => Err(Diag::err(
                format!("Expected expr, found: {:#?}", self.token.kind).to_string(),
                self.token.span,
            )),
        }
    }

    fn parse_ident(&mut self) -> PResult<Ident> {
        let ident = self
            .token
            .ident()
            .ok_or_else(|| Diag::err("Expected ident".to_string(), self.token.span))?;
        self.bump();
        Ok(ident)
    }

    fn parse_param(&mut self) -> PResult<Param> {
        let ident = self.parse_ident()?;
        let ty = self.parse_ty_annot()?;

        Ok(Param { ty, ident })
    }

    fn parse_ty(&mut self) -> PResult<Ty> {
        let ty = match self.token.ident() {
            Some(ident) => match ident.get_symbol() {
                ty::I8 => Ty::new(TyKind::Int(IntTy::I8), self.token.span),
                ty::U8 => Ty::new(TyKind::Uint(UintTy::U8), self.token.span),

                ty::I16 => Ty::new(TyKind::Int(IntTy::I16), self.token.span),
                ty::U16 => Ty::new(TyKind::Uint(UintTy::U16), self.token.span),

                ty::I32 => Ty::new(TyKind::Int(IntTy::I32), self.token.span),
                ty::U32 => Ty::new(TyKind::Uint(UintTy::U32), self.token.span),

                ty::I64 => Ty::new(TyKind::Int(IntTy::I64), self.token.span),
                ty::U64 => Ty::new(TyKind::Uint(UintTy::U64), self.token.span),

                ty::I128 => Ty::new(TyKind::Int(IntTy::I128), self.token.span),
                ty::U128 => Ty::new(TyKind::Uint(UintTy::U128), self.token.span),

                ty::Isize => Ty::new(TyKind::Int(IntTy::Isize), self.token.span),
                ty::Usize => Ty::new(TyKind::Uint(UintTy::Usize), self.token.span),

                _ => {
                    return Err(Diag::err(
                        format!("Expected ty, found {:#?}", self.token.kind).to_string(),
                        self.token.span,
                    ));
                }
            }, // TODO: Rewrite this hard code
            _ => {
                return Err(Diag::err(
                    format!("Expected ty, found {:#?}", self.token.kind).to_string(),
                    self.token.span,
                ));
            }
        };
        self.bump();
        Ok(ty)
    }

    fn parse_fn_params(&mut self) -> PResult<Vec<Param>> {
        if !self.check(TokenKind::OpenParen) {
            return Err(Diag::err("Missing fn params".to_string(), self.token.span));
        }

        self.bump();
        let mut params: Vec<Param> = Vec::new();
        while !matches!(self.token.kind, TokenKind::Eof | TokenKind::CloseParen) {
            params.push(self.parse_param()?);

            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.bump();

        Ok(params)
    }

    fn eat(&mut self, tok: TokenKind) -> bool {
        let is_present = self.check(tok);
        if is_present {
            self.bump()
        };
        is_present
    }

    fn check(&self, tok: TokenKind) -> bool {
        self.token.kind == tok
    }

    fn bump(&mut self) {
        let token = self.token_cursor.next_and_bump();
        self.bump_with(token);
    }

    fn bump_with(&mut self, next_token: Token) {
        self.prev_token = replace(&mut self.token, next_token)
    }

    fn parse_fn_decl(&mut self) -> PResult<FnDecl> {
        Ok(FnDecl {
            params: self.parse_fn_params()?,
            ty: self.parse_ty_annot()?,
        })
    }

    fn parse_let(&mut self) -> PResult<StmtKind> {
        self.bump();

        let ident = self
            .token
            .ident()
            .ok_or_else(|| Diag::err("Expected ident".to_string(), self.token.span))?;

        self.bump();
        let ty = self.parse_ty_annot()?;

        if !self.eat(TokenKind::Eq) {
            return Err(Diag::err("Expected Eq".to_string(), self.token.span));
        }

        let init = self.parse_expr()?;

        if !self.check(TokenKind::Semi) {
            return Err(Diag::err("Expected Semi".to_string(), self.token.span));
        }
        self.bump();

        Ok(StmtKind::Let(
            Box::new(ident),
            Box::new(ty),
            Box::new(init),
            ident.get_span(),
        ))
    }
}

#[derive(Debug)]
struct Fn {
    ident: Ident,
    decl: FnDecl,
    body: Box<Block>,
}

#[derive(Debug)]
enum ItemKind {
    Const(Box<Ident>, Box<Ty>, Box<Expr>, Span),
    Fn(Box<Fn>),
}

#[derive(Debug)]
struct Item {
    kind: ItemKind,
    span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn ty(s: &str) -> Ty {
        parser(s).parse_ty().unwrap()
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
}
