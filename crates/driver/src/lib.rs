mod cli;
mod options;

use clap::Parser;
use cli::Cli;
use err::{Diag, catch_fatal_error_marker, raise};
use lex::flex;
use parser::parse;
use sess::{Options, Session, Source, Stage};
use std::process::ExitCode;

fn unwrap_or_emit_fatal<T>(expr: Result<T, Vec<Diag>>) -> T {
    expr.unwrap_or_else(|errs| {
        for err in errs {
            err.emit();
        }
        raise()
    })
}

fn run() {
    let Cli {
        input,
        emit,
        output,
    } = Cli::parse();

    let sess = Session::new(
        Options {
            emit: emit.map(Stage::from),
            output,
        },
        Source::from(input),
    );

    let stream = unwrap_or_emit_fatal(flex(&sess.source.text));
    let unit = parse(stream);
}

pub fn entry() -> ExitCode {
    catch_fatal_error_marker(run)
}
