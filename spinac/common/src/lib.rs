use std::{collections::HashMap, ops::Range};

use std::sync::LazyLock;
use std::sync::Mutex;

static INTERNER: LazyLock<Mutex<Interner>> = LazyLock::new(|| Mutex::new(Interner::new(SYMBOLS)));

macro_rules! symbols {
    (
        keywords {
            $($kw_name:ident : $kw_str:literal),* $(,)?
        }
        types {
            $($ty_name:ident : $ty_str:literal),* $(,)?
        }
    ) => {
        const SYMBOLS: &[&str] = &[$($kw_str,)* $($ty_str,)*];

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
}

symbols! {
    keywords {
        Let: "let",
        Const: "const",
        If: "if",
        For: "for",
        While: "while",
        Fn: "fn",
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
        F16: "f16",
        F32: "f32",
        F64: "f64",
        F128: "f128",
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Span {
    start: u32,
    end: u32,
}

impl From<Range<usize>> for Span {
    fn from(r: Range<usize>) -> Self {
        Self {
            start: r.start as u32,
            end: r.end as u32,
        }
    }
}

impl Span {
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

struct Interner {
    hs: HashMap<Box<str>, Symbol>,
    strs: Vec<Box<str>>,
}

impl Interner {
    fn new(init: &[&str]) -> Self {
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

    fn intern(&mut self, s: &str) -> Symbol {
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
    const fn new(n: u32) -> Self {
        Self(n)
    }

    pub fn intern(s: &str) -> Self {
        INTERNER.lock().unwrap().intern(s)
    }

    fn as_str(&self) -> String {
        INTERNER.lock().unwrap().get_str(*self).to_owned()
    }
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
