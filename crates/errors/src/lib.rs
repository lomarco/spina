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

pub struct Diag<'a> {
    level: Level,
    message: Vec<String>,
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
