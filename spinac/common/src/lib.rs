use std::{
    ops::Range,
    collections::HashMap,
};

use std::sync::LazyLock;
use std::sync::Mutex;

static INTERNER: LazyLock<Mutex<Interner>> = LazyLock::new(|| {
    Mutex::new(Interner::new(KEYWORDS))
});

macro_rules! scalar {
    ($($name:ident: $str:literal),* $(,)?) => {
        pub const SCALAR: &[&str] = &[$($str),*];
        #[allow(non_upper_case_globals)]
        pub mod ty {
            scalar!(@consts 0u32; $($name), *);
        }
    };
    (@consts $i:expr; $name:ident $(, $rest:ident)*) => {
        pub const $name: crate::Symbol = crate::Symbol::new($i);
        scalar!(@consts $i + 1; $($rest),*);
    };
    (@consts $i:expr;) => {};
}

scalar! {
    I8: "i8",
    U8: "u8",
    I16: "i16",
    U16: "U16",
    I32: "i32",
    U32: "U32",
    I64: "i64",
    U64: "U64",
    I128: "i128",
    U128: "U128",
    Isize: "isize",
    Usize: "usize",
}

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

#[derive(Debug, Copy, Clone, PartialEq)]
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
    hs: HashMap<Box<str>, Symbol>,
    strs: Vec<Box<str>>
}

impl Interner {
    pub fn new(init: &[&str]) -> Self {
        let size = init.len();
        let mut hs: HashMap<Box<str>, Symbol> = HashMap::with_capacity(size);
        let mut strs: Vec<Box<str>> = Vec::with_capacity(size);

        for (i, v) in init.iter().enumerate() {
            let owned: Box<str> = Box::from(*v);

            hs.insert(owned.clone(), Symbol::new(i as u32));
            strs.push(owned);
        }
        Interner { hs, strs }
    }

    pub fn intern(&mut self, s: &str) -> Symbol {
        if let Some(&symbol) = self.hs.get(s) {
            return symbol;
        }

        let id = self.strs.len() as u32;

        let owned: Box<str> = Box::from(s);

        self.hs.insert(owned.clone(), Symbol::new(id)); ////////
        self.strs.push(owned);
        Symbol(id)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Symbol(u32); // TODO: Add Interner as HashTable (id: u32 -> Symbol: String)

impl Symbol {
    pub const fn new(n: u32) -> Self {
        Self(n)
    }

    pub fn intern(s: &str) -> Self {
        INTERNER.lock().unwrap().intern(s)
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
