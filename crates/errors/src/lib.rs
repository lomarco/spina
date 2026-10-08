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
        Diag { level, message, span }
    }

    pub fn emit(self) {
        let level = &self.level;
        self.emit_diagnostic();

        match level {
            Level::Bug => panic_any(ExplicitBug),
            Level::Fatal => raise(),
            _ => {},
        }
    }

    pub fn emit_fatal(self) -> ! {
        assert_eq!(self.level, Level::Fatal);
        self.emit();
        unreachable!();
    }

    fn struct_fatal(self, message: String) -> Diag {
        Diag::new(Level::Fatal, message, None)
    }

    pub fn fatal(self, msg: String) -> ! {
        self.struct_fatal(msg).emit_fatal()
    }

    fn struct_err(self, message: String, span: Span) -> Diag {
        Diag::new(Level::Error, message, Some(span))
    }

    fn emit_err(self) {
        assert_eq!(self.level, Level::Error);
        self.emit();
    }

    pub fn err(self, msg: String, span: Span) {
        self.struct_err(msg, span).emit_err();
    }

    fn emit_diagnostic(&self) {
        eprintln!("Error: {}:, {:#?}", self.message, self.span) // TODO: Add gen diagnostic text with level, ermess, exit-code, span and extract
                   // of code
    }
}
