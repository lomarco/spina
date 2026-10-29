use common::{Ident, IntTy, Lit, Span, Ty, UintTy, kw, ty};
use err::{Diag, PResult, raise};
use lex::{Token, TokenCursor, TokenKind, TokenStream, flex};
use sess::{Input, Session};
use std::{fs::read_to_string, io::ErrorKind, mem::replace, path::Path};

#[derive(Debug)]
pub struct Unit(pub Vec<Item>);

#[derive(Debug)]
pub struct Param {
    pub ty: Ty,
    pub ident: Ident,
}

#[derive(Debug)]
pub struct FnDecl {
    pub params: Vec<Param>,
    pub ty: Ty,
}

#[derive(Debug)]
pub struct ForLoop {
    pub ident: Box<Ident>,
    pub iter: Box<Expr>,
    pub body: Box<Block>,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug)]
pub enum BinOpKind {
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
pub struct BinOp {
    pub kind: BinOpKind,
    pub span: Span,
}

impl BinOp {
    fn new(kind: BinOpKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub enum UnOp {
    /// The `*` operator for dereferencing
    Deref,
    /// The `!` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub enum StmtKind {
    Let(Box<Ident>, Box<Ty>, Box<Expr>, Span),
    Semi(Box<Expr>),
    Expr(Box<Expr>),
    Empty,
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum ExprKind {
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

pub fn unwrap_or_emit_fatal<T>(expr: Result<T, Vec<Diag>>) -> T {
    expr.unwrap_or_else(|errs| {
        for err in errs {
            err.emit();
        }
        raise()
    })
}

pub fn new_parser_from_file(path: &Path) -> Result<Parser, Vec<Diag>> {
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

pub struct Parser {
    pub token: Token,
    pub prev_token: Token,
    token_cursor: TokenCursor,
}

impl Parser {
    pub fn new(stream: TokenStream) -> Self {
        let mut parser = Parser {
            token: Token::dummy(),
            prev_token: Token::dummy(),
            token_cursor: TokenCursor::new(stream),
        };

        parser.bump();

        parser
    }

    pub fn parse_unit(&mut self) -> PResult<Unit> {
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
            // TODO: Add const item parse
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
            return Err(Diag::err(format!("Expected Semi, found {:#?}", self.token.kind), self.token.span));
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
                self.bump();
                Ok(Expr::new(
                    ExprKind::Ident(Ident {
                        name,
                        span: self.token.span,
                    }),
                    self.token.span,
                ))
            }

            TokenKind::FloatLiteral(flit) => {
                self.bump();
                Ok(Expr::new(ExprKind::Lit(flit), self.token.span))
            }

            TokenKind::IntLiteral(ilit) => {
                self.bump();
                Ok(Expr::new(ExprKind::Lit(ilit), self.token.span))
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
            Some(ident) => match ident.name {
                ty::I8 => Ty::new(common::TyKind::Int(IntTy::I8), self.token.span),
                ty::U8 => Ty::new(common::TyKind::Uint(UintTy::U8), self.token.span),

                ty::I16 => Ty::new(common::TyKind::Int(IntTy::I16), self.token.span),
                ty::U16 => Ty::new(common::TyKind::Uint(UintTy::U16), self.token.span),

                ty::I32 => Ty::new(common::TyKind::Int(IntTy::I32), self.token.span),
                ty::U32 => Ty::new(common::TyKind::Uint(UintTy::U32), self.token.span),

                ty::I64 => Ty::new(common::TyKind::Int(IntTy::I64), self.token.span),
                ty::U64 => Ty::new(common::TyKind::Uint(UintTy::U64), self.token.span),

                ty::I128 => Ty::new(common::TyKind::Int(IntTy::I128), self.token.span),
                ty::U128 => Ty::new(common::TyKind::Uint(UintTy::U128), self.token.span),

                ty::Isize => Ty::new(common::TyKind::Int(IntTy::Isize), self.token.span),
                ty::Usize => Ty::new(common::TyKind::Uint(UintTy::Usize), self.token.span),

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
        if !self.eat(TokenKind::Colon) {
            return Err(Diag::err(
                format!("Expected Colon, found {:?}", self.token.kind).to_string(),
                self.token.span,
            ));
        }

        let ty = self.parse_ty()?;

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
            ident.span,
        ))
    }
}

#[derive(Debug)]
pub struct Fn {
    pub ident: Ident,
    pub decl: FnDecl,
    pub body: Box<Block>,
}

#[derive(Debug)]
pub enum ItemKind {
    Const(Box<Ident>, Box<Ty>, Box<Expr>, Span),
    Fn(Box<Fn>),
}

#[derive(Debug)]
pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}
