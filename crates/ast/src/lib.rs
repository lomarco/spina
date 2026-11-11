use span::Span;
use symbol::{Symbol, ty};

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
struct ForLoop {
    ident: Box<Ident>,
    iter: Box<Expr>,
    body: Box<Block>,
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
    pub fn new(kind: BinOpKind, span: Span) -> Self {
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
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
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

#[derive(Debug, Clone, Copy)]
pub struct Ident {
    name: Symbol,
    span: Span,
}

impl Ident {
    pub fn new(name: Symbol, span: Span) -> Self {
        Ident { name, span }
    }

    pub fn get_symbol(&self) -> Symbol {
        self.name
    }

    pub fn get_name_str(&self) -> String {
        self.name.as_str()
    }

    pub fn get_span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq)]
pub enum FloatTy {
    F16,
    F32,
    F64,
    F128,
}

#[derive(Debug, PartialEq)]
pub enum UintTy {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Debug, PartialEq)]
pub enum IntTy {
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Debug, PartialEq)]
pub enum TyKind {
    Array(Box<Ty>, u32),
    Ptr(Box<Ty>),
    Int(IntTy),
    Uint(UintTy),
    Float(FloatTy),
    Str,
    Bool,
    Char,

    Dummy,
}

impl TyKind {
    pub fn from_symbol(symbol: Symbol) -> Option<Self> {
        Some(match symbol {
            ty::I8 => TyKind::Int(IntTy::I8),
            ty::U8 => TyKind::Uint(UintTy::U8),

            ty::I16 => TyKind::Int(IntTy::I16),
            ty::U16 => TyKind::Uint(UintTy::U16),

            ty::I32 => TyKind::Int(IntTy::I32),
            ty::U32 => TyKind::Uint(UintTy::U32),

            ty::I64 => TyKind::Int(IntTy::I64),
            ty::U64 => TyKind::Uint(UintTy::U64),

            ty::I128 => TyKind::Int(IntTy::I128),
            ty::U128 => TyKind::Uint(UintTy::U128),

            ty::Isize => TyKind::Int(IntTy::Isize),
            ty::Usize => TyKind::Uint(UintTy::Usize),

            ty::F16 => TyKind::Float(FloatTy::F16),
            ty::F32 => TyKind::Float(FloatTy::F32),
            ty::F64 => TyKind::Float(FloatTy::F64),
            ty::F128 => TyKind::Float(FloatTy::F128),

            _ => return None,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Ty {
    kind: TyKind,
    span: Span,
}

impl Ty {
    pub fn new(kind: TyKind, span: Span) -> Self {
        Ty { kind, span }
    }

    pub fn get_kind(&self) -> &TyKind {
        &self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lit {
    Char(char),
    Integer(u128),
    Float(f64),
    Str(Symbol),
}
