use std::{
    fs::read_to_string,
    path::Path,
    mem::replace,
    io::ErrorKind,
};
use common::{Span, Ident, Ty, Symbol};
use lex::{Token, TokenStream, TokenCursor, TokenKind, flex};
use session::{Session, ParseSess, Input};
use errors::{Diag, PResult};

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
    Let(Box<Ident>, Box<Expr>, Span),
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

pub fn new_parser_from_file(psess: &ParseSess, path: &Path, sp: Option<Span>) -> Result<Parser, Vec<Diag>> {
    let cont = read_to_string(path).map_err(|e| {
        let msg = match e.kind() {
            ErrorKind::NotFound => format!("couldn't find file `{}`", path.display()),
            ErrorKind::PermissionDenied => format!("permission denied when opening file `{}`", path.display()),
            ErrorKind::IsADirectory => format!("`{}` is a directory", path.display()),
            _ => format!("couldn't read `{}`: {}", path.display(), e),
        };

        Diag::fatal(msg)
    });

    let stream = flex(cont.as_str())?;

    let parser = Parser::new(psess, stream);
    Ok(parser)
}

fn new_parser_from_str(psess: &ParseSess, str: &String) -> Result<Parser, Vec<Diag>> {
    Ok(Parser::new(psess, flex(str)?))
}

pub fn parse(sess: &Session) -> Unit { // TODO: Add new_parser_from_source_str
    unwrap_or_emit_fatal(match &sess.input {
        Input::File(file) => new_parser_from_file(&sess.psess, file, None),
        Input::Str(str) => new_parser_from_str(&sess.psess, str),
    }).parse_unit().unwrap_or_else(|parse_error| parse_error.emit().raise())
}

pub struct Parser {
    psess: &ParseSess,
    pub token: Token,
    token_cursor: TokenCursor,
}

impl Parser {
    pub fn new(psess: &ParseSess, stream: TokenStream) -> Self {
        Parser {
            psess,
            token: Token::dummy(),
            token_cursor: TokenCursor::new(stream),
        }
    }

    pub fn parse_unit(&self) -> PResult<Unit> {
        Ok(Unit(parse_items()))
    }

    fn parse_items(&self) -> PResult<Vec<Item>> {
        let items = Vec::with_capacity(128);

        loop {
            let Some(item) = self.parse_item()? else {
                break Ok(items);
            };
            items.push(item);
        }
    }

    fn parse_fn(&self, sp: Span) -> PResult<(Ident, FnDecl, Box<Block>)> {
        let fn_span = self.token.span;
        let ident = self.parse_ident()?;
        let decl = self.parse_fn_decl()?;
        let body = self.parse_fn_body()?;
        Ok((ident, decl, body))
    }

    fn parse_ident(&self) -> PResult<Ident> {
        let ident = self.token.ident().ok_or_else(|| "Expected ident".to_string())?;
        self.bump();
        Ok(ident)
    }

    fn parse_param(&self) -> PResult<Param> {
        let ident = self.parse_fn_param_ident_colon()?;
        let ty = self.parse_ty()?;

        Ok(Param { ty, ident })
    }

    fn parse_fn_param_ident_colon(&self) -> PResult<Ident> {
        let ident = self.parse_ident()?;
        if !self.eat(TokenKind::Colon) {
            return Err(Diag::err("Expected colon".as_string(), self.token.span));
        }
        Ok(ident)
    }

    fn parse_ty(&self) -> PResult<Ty> {
        let ty = self.token.ty().ok_or_else(|| Diag::err("Expected ty".to_string(), self.token.span))?;
        self.bump();
        Ok(ty)
    }

    fn parse_fn_params(&self) -> PResult<Vec<Param>> {
        if self.token != TokenKind::OpenParen {
            return Err(Diag::err("Missing fn params".as_string(), self.token.span));
        }
        let params: Vec<Param> = Vec::new();
        while (token.kind != TokenKind::Eof || token.kind != TokenKind::CloseParen) {
            params.push(parse_param()?);

            if !self.eat(TokenKind::Comma) {
                break;
            }
        }

        Ok(params)
    }

    fn eat(&self, tok: TokenKind) -> bool {
        let is_present = check(tok);
        if is_present {
            self.bump()
        };
        is_present
    }

    fn check(&self, tok: TokenKind) -> bool {
        self.token == tok
    }

    fn bump(&self) {
        self.bump_with(self.token_cursor.next_and_bump())
    }

    fn bump_with(&self, next_token: Token) {
        self.prev_token = replace(&mut self.token, next_token)
    }

    fn parse_fn_decl(&self) -> PResult<FnDecl> {
        Ok(FnDecl {
            params: self.parse_fn_params()?,
            ty: self.parse_ty()?,
        })
    }
    fn parse_let(&self) -> PResult<Expr> {
        if !eat(TokenKind::Ident) {
            return Err(Diag::err("Expected ident".to_string(), self.token.span));
        }
        let ident = self.token.ident().ok_or_else(|| Diag::err("Expected ident".to_string(), self.token.span))?;

        if !eat(TokenKind::Colon) {
            return Err(Diag::err("Expected Colon".to_string(), self.token.span));
        }
        self.bump();

        let ty = self.parse_ty()?;

        if !eat(TokenKind::Eq) {
            return Err(Diag::err("Expected Eq".to_string(), self.token.span));
        }

        self.bump();

        let init = parse_init();
        Ok(Expr {
            kind: ExprKind::Let(Box::new(ident),
                Box::new(init),
                ident.span),
            span: ident.span // FIXME: Replace it to real span
        })
    }
}

pub struct ConstItem {
    pub ident: Ident,
    pub ty: Box<Ty>,
    pub body: Option<Box<Expr>>,
}

pub struct Fn {
    pub ident: Ident,
    pub ty: Box<Ty>,
    pub body: Option<Box<Block>>,
}

pub enum ItemKind {
    Const(Box<ConstItem>),
    Fn(Box<Fn>),
}

pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}
