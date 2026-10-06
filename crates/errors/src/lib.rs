use std::panic::{panic_any, resume_unwind};

pub type PResult<'a, T> = Result<T, Diag<'a>>;

pub struct ExplicitBug;
pub struct FatalErrorMarker;

#[derive(Debug, PartialEq)]
pub enum Level {
    Bug,
    Fatal,
    Error,
    Note,
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
