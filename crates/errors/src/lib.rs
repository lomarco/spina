use std::panic::{panic_any, resume_unwind};
use common::Span;

pub type PResult<T> = Result<T, Diag>;

pub struct ExplicitBug;
pub struct FatalErrorMarker;

#[derive(Debug, PartialEq)]
pub enum Level {
    Bug,
    Fatal,
    Error,
    Note,
}

pub struct Diag {
    level: Level,
    message: String,
    code: u32,
    span: Span,
}

impl Diag {
    pub fn new(level: Level, message: String, code: u32, span: Span) -> Self {
        Self { level, message, code, span }
    }

    fn raise(self) -> ! {
        resume_unwind(Box::new(FatalErrorMarker));
    }

    pub fn emit(self) {
        let level = &self.level;
        self.emit_diagnostic();

        match level {
            Level::Bug => panic_any(ExplicitBug),
            Level::Fatal => self.raise(),
            _ => {},
        }
    }

    pub fn emit_fatal(self) -> ! {
        assert_eq!(self.level, Level::Fatal);
        self.emit();
        unreachable!();
    }
}
