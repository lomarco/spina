use common::{Span, Spanned};
use lex::Token;

pub struct Unit {
    pub items: Vec<Item>,
}

pub struct Ident {
    pub name: Symbol,
    pub span: Span,
}

pub struct Symbol(SymbolIndex);

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

pub struct Lit {
    pub kind: LitKind,
    pub symbol: Symbol,
    pub suffix: Option<Symbol>,
}

pub struct ForLoop {
    pub pat: Box<Pat>,
    pub iter: Box<Expr>,
    pub body: Box<Block>,
}

pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

pub enum PatKind {
    Expr(Box<Expr>),
}

pub struct Pat {
    pub kind: PatKind,
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

pub type BinOp = Spanned<BinOpKind>;

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
    Tup(Vec<Box<Expr>>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Lit(token::Lit),
    Let(Box<Pat>, Box<Expr>, Span),
    If(Box<Expr>, Box<Block>, Option<Box<Expr>>),
    While(Box<Expr>, Box<Block>),
    ForLoop(Box<ForLoop>),
    Loop(Box<Block>, Span),
    Block(Box<Block>),
    Field(Box<Expr>, Ident),
    Index(Box<Expr>, Box<Expr>, Span),
    Break(Option<Box<Expr>>),
    Continue(), // FIXME: Add Label
    Ret(Option<Box<Expr>>),
}

pub struct Parser<'a> {
    tokens: Vec<Spanned<Token<'a>>>,
}

pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}
