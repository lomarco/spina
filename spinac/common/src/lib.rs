use std::{
    ops::Range,
    collections::HashMap,
};

use std::sync::LazyLock;

pub static INTERNER: LazyLock<Interner> = LazyLock::new(|| {
    Interner::new(KEYWORDS)
});

macro_rules! keywords {
    ($($name:ident: $str:literal),* $(,)?) => {
        pub const KEYWORDS: &[&str] = &[$($str),*];
        #[allow(non_upper_case_globals)]
        pub mod kw {
            keywords!(@consts 0u32; $($name), *);
        }
    };
    (@consts $i:expr; $name:ident $(, $rest:ident)*) => {
        pub const $name: crate::Symbol = crate::Symbol::new($i);
        keywords!(@consts $i + 1; $($rest),*);
    };
    (@consts $i:expr;) => {};
}

keywords! {
    Let: "let",
    If: "if",
    For: "for",
    While: "while",
    // TODO: Add the rest keywords
    // ...
}

#[derive(Debug, Copy, Clone)]
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

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

pub struct Interner {
    hs: HashMap<&'static str, Symbol>,
}

impl Interner {
    pub fn new(init: &[&'static str]) -> Self {
        let hs_size = init.len();
        let mut hs: HashMap<&str, u32> = HashMap::with_capacity(hs_size);

        for v in init {
            hs.entry(v);
        }
        Interner { hs } // TODO: Add key as index from 1 to ...
    }

    pub fn intern(&self, s: &str) -> Symbol {
        ;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol(u32); // TODO: Add Interner as HashTable (id: u32 -> Symbol: String)

impl Symbol {
    pub const fn new(n: u32) -> Self {
        Self(n)
    }

    pub fn intern(s: &str) -> Self {
        INTERNER.intern(s)
    }
}

pub struct Ident {
    pub name: Symbol,
    pub span: Span,
}

impl Ident {
    pub fn new(name: Symbol, span: Span) -> Self {
        Ident { name, span }
    }
}

pub enum FloatTy {
    F16,
    F32,
    F64,
    F128,
}

pub enum UintTy {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

pub enum IntTy {
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

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

pub struct Ty {
    pub kind: TyKind,
    pub span: Span,
}

impl Ty {
    pub fn new(kind: TyKind, span: Span) -> Self {
        Ty { kind, span }
    }
}
