use std::{
    ops::Range,
    collections::HashMap,
};

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

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol(u32); // TODO: Add Interner as HashTable (id: u32 -> Symbol: String)

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
