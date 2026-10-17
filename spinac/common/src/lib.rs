use std::{
    ops::Range,
    collections::HashMap,
};

use std::sync::LazyLock;
use std::sync::Mutex;

static INTERNER: LazyLock<Mutex<Interner>> = LazyLock::new(|| {
    Mutex::new(Interner::new(SYMBOLS))
});

macro_rules! symbols {
    (
        keywords {
            $($kw_name:ident : $kw_str:literal),* $(,)?
        }
        types {
            $($ty_name:ident : $ty_str:literal),* $(,)?
        }
    ) => {
        pub const SYMBOLS: &[&str] = &[$($kw_str,)* $($ty_str,)*];

        #[allow(non_upper_case_globals)]
        pub mod kw {
            symbols!(@consts 0u32; $($kw_name), *);
        }

        #[allow(non_upper_case_globals)]
        pub mod ty {
            symbols!(
                @consts
                symbols!(@count 0u32; $($kw_name), *);
                $($ty_name), *
            );
        }
    };

    (@consts $i:expr; $name:ident $(, $rest:ident)*) => {
        pub const $name: crate::Symbol = crate::Symbol::new($i);
        symbols!(@consts $i + 1; $($rest),*);
    };
    (@consts $i:expr;) => {};

    (@count $i:expr; $name:ident $(, $rest:ident)*) => {
        symbols!(@count $i + 1; $($rest),*)
    };
    (@count $i:expr;) => {
        $i
    };
} // TODO: Fix symbols order

symbols! {
    keywords {
        Let: "let",
        If: "if",
        For: "for",
        While: "while",
        Fn: "fn",
        // TODO: Add the rest keywords
        // // ...
    }

    types {
        I8: "i8",
        U8: "u8",
        I16: "i16",
        U16: "u16",
        I32: "i32",
        U32: "u32",
        I64: "i64",
        U64: "u64",
        I128: "i128",
        U128: "u128",
        Isize: "isize",
        Usize: "usize",
    }
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

        self.hs.insert(owned.clone(), Symbol::new(id));
        self.strs.push(owned);
        Symbol(id)
    }

    fn get_str(&self, symbol: Symbol) -> &str {
        &self.strs[symbol.0 as usize]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Symbol(u32);

impl Symbol {
    pub const fn new(n: u32) -> Self {
        Self(n)
    }

    pub fn intern(s: &str) -> Self {
        INTERNER.lock().unwrap().intern(s)
    }

    pub fn as_str(&self) -> String {
        INTERNER.lock().unwrap().get_str(*self).to_owned()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ident {
    pub name: Symbol,
    pub span: Span,
}

impl Ident {
    pub fn new(name: Symbol, span: Span) -> Self {
        Ident { name, span }
    }
}

#[derive(Debug)]
pub enum FloatTy {
    F16,
    F32,
    F64,
    F128,
}

#[derive(Debug)]
pub enum UintTy {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Debug)]
pub enum IntTy {
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Ty {
    pub kind: TyKind,
    pub span: Span,
}

impl Ty {
    pub fn new(kind: TyKind, span: Span) -> Self {
        Ty { kind, span }
    }
}
