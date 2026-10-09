use std::panic::{panic_any, resume_unwind};
use common::Span;
use std::process::ExitCode;
use std::panic::catch_unwind;
use std::panic::AssertUnwindSafe;

pub type PResult<T> = Result<T, Diag>;

pub struct ExplicitBug;
pub struct FatalErrorMarker;

pub fn raise() -> ! {
    resume_unwind(Box::new(FatalErrorMarker));
}

pub fn catch_fatal_error_marker<T>(f: impl FnOnce() -> T) -> ExitCode {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(_) => ExitCode::SUCCESS,
        Err(val) => {
            if !val.is::<FatalErrorMarker>() {
                resume_unwind(val);
            }
            ExitCode::FAILURE
        }
    }
}

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
    span: Option<Span>,
}

impl Diag {
    fn new(level: Level, message: String, span: Option<Span>) -> Self {
        Self { level, message, span }
    }

    pub fn err(message: String, span: Span) -> Self {
        Self::new(Level::Error, message, Some(span))
    }

    pub fn fatal(message: String) -> ! {
        Self::new(Level::Fatal, message, None).emit();
        unreachable!();
    }

    pub fn emit(&self) {
        eprintln!("Error: {}:, {:#?}", self.message, self.span); // TODO: Add gen diagnostic text with level, ermess, exit-code, span and extract
                   // of code
        if self.level == Level::Fatal {
            raise()
        }
    }
}
