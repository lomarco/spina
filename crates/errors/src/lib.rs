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

impl Diag {
    pub fn raise(self) -> ! {
        resume_unwind(Box::new(FatalErrorMarker));
    }
    pub fn emit(self) {
        let level = self.level;
        self.dcx.emit_diagnostic(self.take_diag());

        match level {
            Level::Bug => panic_any(ExplicitBug),
            Level::Fatal => self.raise(),
            _ => {}
        }
    }

    pub fn emit_fatal(self) -> ! {
        assert_eq!(self.level, Level::Fatal);
        self.emit();
        unreachable!();
    }
}
