mod cli;
mod options;

use clap::Parser;
use cli::Cli;
use err::{catch_fatal_error_marker, Diag, raise};
use lex::flex;
use parser::parse;
use sess::Session;
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
    let lex = flex(match sess.opts.input {
        sess::Input::Str(s) => s.as_str(),
        sess::Input::File(path) => read_to_string(path)
            .unwrap_or_else(|e| {
                let msg = match e.kind() {
                    ErrorKind::NotFound => format!("couldn't find file `{}`", path.display()),
                    ErrorKind::PermissionDenied => {
                        format!("permission denied when opening file `{}`", path.display())
                    }
                    ErrorKind::IsADirectory => format!("`{}` is a directory", path.display()),
                    _ => format!("couldn't read `{}`: {}", path.display(), e),
                };

                Diag::fatal(msg)
            })
            .as_str(),
    });

    let unit = parse(unwrap_or_emit_fatal(lex));
}

pub fn entry() -> ExitCode {
    catch_fatal_error_marker(run)
}
