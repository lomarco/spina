mod cli;
mod options;

use clap::Parser;
use cli::Cli;
use err::{Diag, catch_fatal_error_marker, raise};
use lex::flex;
use parser::parse;
use sess::{Input, Session};
use std::process::ExitCode;
use std::{fs::read_to_string, io::ErrorKind};

fn unwrap_or_emit_fatal<T>(expr: Result<T, Vec<Diag>>) -> T {
    expr.unwrap_or_else(|errs| {
        for err in errs {
            err.emit();
        }
        raise()
    })
}

fn run() {
    let sess = Session::new(Cli::parse().into());

    let lex = match &sess.opts.input {
        Input::File(path) => read_to_string(path).unwrap_or_else(|e| {
            Diag::fatal(match e.kind() {
                ErrorKind::NotFound => format!("couldn't find file `{}`", path.display()),
                ErrorKind::PermissionDenied => {
                    format!("permission denied when opening file `{}`", path.display())
                }
                ErrorKind::IsADirectory => format!("`{}` is a directory", path.display()),
                _ => format!("couldn't read `{}`: {}", path.display(), e),
            })
        }),
        Input::Str(code) => code.clone(),
    };
    let stream = unwrap_or_emit_fatal(flex(&lex));

    // TODO: Feed `unit` into the optimizer and codegen.
    let unit = parse(stream);
}

pub fn entry() -> ExitCode {
    catch_fatal_error_marker(run)
}
