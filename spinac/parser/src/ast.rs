use common::{Ident, Lit, Span, Ty};

#[derive(Debug)]
pub struct Unit(pub(crate) Vec<Item>);

#[derive(Debug)]
pub(crate) struct Param {
    pub(crate) ty: Ty,
    pub(crate) ident: Ident,
}

#[derive(Debug)]
pub(crate) struct FnDecl {
    pub(crate) params: Vec<Param>,
    pub(crate) ty: Ty,
}

#[derive(Debug)]
struct ForLoop {
    ident: Box<Ident>,
    iter: Box<Expr>,
    body: Box<Block>,
}

#[derive(Debug)]
pub(crate) struct Block {
    pub(crate) stmts: Vec<Stmt>,
    pub(crate) span: Span,
}

#[derive(Debug)]
pub(crate) enum BinOpKind {
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
pub(crate) struct BinOp {
    kind: BinOpKind,
    pub(crate) span: Span,
}

impl BinOp {
    pub(crate) fn new(kind: BinOpKind, span: Span) -> Self {
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
pub(crate) struct Expr {
    kind: ExprKind,
    span: Span,
}

impl Expr {
    pub(crate) fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub(crate) enum StmtKind {
    Let(Box<Ident>, Box<Ty>, Box<Expr>, Span),
    Semi(Box<Expr>),
    Expr(Box<Expr>),
    Empty,
}

#[derive(Debug)]
pub(crate) struct Stmt {
    pub(crate) kind: StmtKind,
    pub(crate) span: Span,
}

#[derive(Debug)]
pub(crate) enum ExprKind {
    Array(Vec<Expr>),
    Call(Ident, Vec<Expr>),
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

#[derive(Debug)]
pub(crate) struct Fn {
    pub(crate) ident: Ident,
    pub(crate) decl: FnDecl,
    pub(crate) body: Box<Block>,
}

#[derive(Debug)]
pub(crate) enum ItemKind {
    Const(Box<Ident>, Box<Ty>, Box<Expr>, Span),
    Fn(Box<Fn>),
}

#[derive(Debug)]
pub(crate) struct Item {
    pub(crate) kind: ItemKind,
    pub(crate) span: Span,
}
