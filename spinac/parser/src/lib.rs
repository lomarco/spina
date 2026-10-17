use std::{
    fs::read_to_string,
    path::Path,
    mem::replace,
    io::ErrorKind,
};
use common::{Span, Ident, Ty, Symbol, ty, UintTy, IntTy, kw};
use lex::{Token, TokenStream, TokenCursor, TokenKind, flex};
use sess::{Session, Input};
use err::{Diag, PResult, raise};

// TODO: Add dcx

pub struct Unit(Vec<Item>);

pub enum LitKind {
    Bool, // AST only, must never appear in a `Token`
    Byte,
    Char,
    Integer, // e.g. `1`, `1u8`, `1f32`
    Float,   // e.g. `1.`, `1.0`, `1e3f32`
    Str,
    StrRaw(u8), // raw string delimited by `n` hash symbols
    ByteStr,
    ByteStrRaw(u8), // raw byte string delimited by `n` hash symbols
    CStr,
    CStrRaw(u8),
}

pub struct Param {
    pub ty: Ty,
    pub ident: Ident,
}

pub struct FnDecl {
    pub params: Vec<Param>,
    pub ty: Ty,
}

pub struct Lit {
    pub kind: LitKind,
    pub symbol: Symbol,
    pub suffix: Option<Symbol>,
}

pub struct ForLoop {
    pub ident: Box<Ident>,
    pub iter: Box<Expr>,
    pub body: Box<Block>,
}

pub struct Block {
    pub exprs: Vec<Expr>,
    pub span: Span,
}

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

pub struct BinOp {
    kind: BinOpKind,
    span: Span
}

pub enum UnOp {
    /// The `*` operator for dereferencing
    Deref,
    /// The `!` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
}

pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

pub enum ExprKind {
    Array(Vec<Box<Expr>>),
    Call(Box<Expr>, Vec<Box<Expr>>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Let(Box<Ident>, Box<Ty>, Box<Expr>, Span),
    If(Box<Expr>, Box<Block>, Option<Box<Expr>>),
    While(Box<Expr>, Box<Block>),
    Index(Box<Expr>, Box<Expr>, Span),
    Break(Option<Box<Expr>>),
    Ret(Option<Box<Expr>>),
    Continue(), // FIXME: Add Label

    Lit(Lit),
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
    let cont = read_to_string(path).unwrap_or_else(|e| { // TODO: Maybe delete unwrap_or_else
        let msg = match e.kind() {
            ErrorKind::NotFound => format!("couldn't find file `{}`", path.display()),
            ErrorKind::PermissionDenied => format!("permission denied when opening file `{}`", path.display()),
            ErrorKind::IsADirectory => format!("`{}` is a directory", path.display()),
            _ => format!("couldn't read `{}`: {}", path.display(), e),
        };

        Diag::fatal(msg)
    });

    let stream = flex(&cont)?;

    let parser = Parser::new(stream);
    Ok(parser)
}

fn new_parser_from_str(str: &String) -> Result<Parser, Vec<Diag>> {
    Ok(Parser::new(flex(str)?))
}

pub fn parse(sess: &Session) -> Unit { // TODO: Add new_parser_from_source_str
    unwrap_or_emit_fatal(match &sess.input {
        Input::File(file) => new_parser_from_file(file),
        Input::Str(str) => new_parser_from_str(str),
    }).parse_unit().unwrap_or_else(|parse_error| parse_error.emit_fatal())
}

pub struct Parser {
    pub token: Token,
    pub prev_token: Token,
    token_cursor: TokenCursor,
}

impl Parser {
    pub fn new(stream: TokenStream) -> Self {
        Parser {
            token: Token::dummy(),
            prev_token: Token::dummy(),
            token_cursor: TokenCursor::new(stream),
        }
    }

    pub fn parse_unit(&self) -> PResult<Unit> {
        Ok(Unit(self.parse_items()?))
    }

    fn parse_items(&self) -> PResult<Vec<Item>> {
        let mut items = Vec::with_capacity(128);

        loop {
            let Some(item) = self.parse_item()? else {
                break Ok(items);
            };
            items.push(item);
        }
    }

    fn parse_item(&self) -> PResult<Option<Item>> {
        let span = self.token.span;
        let kind = self.parse_item_kind(span)?;
        Ok(kind.map(|kind| Item { kind, span }))
    }

    fn parse_item_kind(&mut self, sp: Span) -> PResult<Option<ItemKind>> {
        if self.is_fn() {
            let (ident, decl, ty, body) = self.parse_fn()?;
            Ok(Some(ItemKind::Fn(Box::new(Fn {
                ident,
                decl,
                ty,
                body,
            }))))
        } else { // TODO: Add const item parse
            Ok(None)
        }
    }

    fn parse_fn(&mut self) -> PResult<(Ident, FnDecl, Box<Block>)> {
    fn parse_fn(&mut self) -> PResult<(Ident, FnDecl, Ty, Box<Block>)> {
        let ident = self.parse_ident()?;
        let decl = self.parse_fn_decl()?;
        let ty = self.parse_ty_annot()?;
        let body = self.parse_fn_body()?;
        Ok((ident, decl, ty, body))
    }

    fn parse_fn_body(&mut self) -> PResult<Box<Block>> {
        if !self.eat(TokenKind::OpenBrace) {
            return Err(Diag::err("Missing fn OpenBrace".to_string(), self.token.span));
        }

        let mut exprs: Vec<Expr> = Vec::with_capacity(1024);

        self.bump();

        while self.token.kind != TokenKind::Eof
            && self.token.kind != TokenKind::CloseBrace
            && self.token.kind != TokenKind::Semi {
                exprs.push(self.parse_expr()?);
        }

        Ok(Box::new(Block { exprs, span: self.token.span }))
    }

    fn parse_expr(&self) -> PResult<Expr> { // TODO: Add Interner
        Ok(match self.token.kind {
            TokenKind::Ident.is_keyword(kw::Let) => parse_let()?,
            // TODO: Add the rest ExprKind::*
        })
    }

    fn parse_ident(&mut self) -> PResult<Ident> {
        let ident = self.token.ident().ok_or_else(|| Diag::err("Expected ident".to_string(), self.token.span))?;
        self.bump();
        Ok(ident)
    }

    fn parse_param(&mut self) -> PResult<Param> {
        let ident = self.parse_fn_param_ident_colon()?;
        let ty = self.parse_ty()?;

        Ok(Param { ty, ident })
    }

    fn parse_fn_param_ident_colon(&mut self) -> PResult<Ident> {
        let ident = self.parse_ident()?;
        if !self.eat(TokenKind::Colon) {
            return Err(Diag::err("Expected colon".to_string(), self.token.span));
        }
        Ok(ident)
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

                _ => return Err(Diag::err("Expected ty".to_string(), self.token.span))
            } // TODO: Rewrite this hard code
            _ => return Err(Diag::err("Expected ty".to_string(), self.token.span))
        };
        self.bump();
        Ok(ty)
    }

    fn parse_fn_params(&mut self) -> PResult<Vec<Param>> {
        if self.token.kind != TokenKind::OpenParen {
            return Err(Diag::err("Missing fn params".to_string(), self.token.span));
        }

        let mut params: Vec<Param> = Vec::new();
        while self.token.kind != TokenKind::Eof || self.token.kind != TokenKind::CloseParen {
            params.push(self.parse_param()?);

            if !self.eat(TokenKind::Comma) {
                break;
            }
        }

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
            ty: self.parse_ty()?,
        })
    }
    fn parse_let(&mut self) -> PResult<Expr> {
        self.bump();
        let ident = self.token.ident().ok_or_else(|| Diag::err("Expected ident".to_string(), self.token.span))?;

        if !self.eat(TokenKind::Colon) {
            return Err(Diag::err("Expected Colon".to_string(), self.token.span));
        }
        self.bump();

        let ty = self.parse_ty()?;

        if !self.eat(TokenKind::Eq) {
            return Err(Diag::err("Expected Eq".to_string(), self.token.span));
        }

        self.bump();

        let init = self.parse_init()?;
        Ok(Expr {
            kind: ExprKind::Let(Box::new(ident),
                Box::new(ty),
                Box::new(init),
                ident.span),
            span: ident.span // FIXME: Replace it to real span
        })
    }

    fn parse_init(&self) -> PResult<Expr> {
        ;
    }
}

pub struct ConstItem {
    pub ident: Ident,
    pub ty: Box<Ty>,
    pub body: Option<Box<Expr>>,
}

pub struct Fn {
    pub ident: Ident,
    pub decl: FnDecl,
    pub ty: Ty,
    pub body: Box<Block>,
}

pub enum ItemKind {
    Const(Box<ConstItem>),
    Fn(Box<Fn>),
}

pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}
