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
