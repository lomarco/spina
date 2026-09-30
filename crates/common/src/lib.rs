use std::ops::Range;

#[derive(Debug)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl From<Range<usize>> for Span {
    fn from(r: Range<usize>) -> Self {
        Self {
            start: r.start as u32,
            end: r.end as u32
        }
    }
}

pub struct Spanned<T> {
    pub t: T,
    pub span: Span,
}

pub struct Symbol(SymbolIndex);

pub struct Ident {
    pub name: Symbol,
    pub span: Span,
}

impl Ident {
    pub fn new(name: Symbol, span: Span) -> Self {
        Ident { name, span }
    }
}

pub enum TyKind { // FIXME: Add primitives types
    /// A fixed length array (`[T; n]`).
    Array(Box<Ty>, AnonConst),
    /// A raw pointer (`*const T` or `*mut T`).
    Ptr(MutTy),
    /// Placeholder for a kind that has failed to be defined.
    Err(ErrorGuaranteed),
}

pub struct Ty {
    pub kind: TyKind,
    pub span: Span,
}

impl Ty {
    pub fn new(kind: TyKind, span: Span) -> Self {
        Ty { kind, span }
    }
}
