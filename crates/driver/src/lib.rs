mod cli;
mod options;

use clap::Parser;
use cli::Cli;
use err::{Diag, catch_fatal_error_marker, raise};
use lex::{TokenStream, flex};
use parser::{Unit, parse};
use resolve::resolve;
use sess::{Options, Session, Source, Stage};
use std::process::ExitCode;

#[derive(Debug)]
pub enum CompileOutput {
    Tokens(TokenStream),
    Ast(Unit),
}

pub fn compile(sess: &Session) -> Result<CompileOutput, Vec<Diag>> {
    let stream = flex(&sess.source.text)?;

    if let Some(Stage::Tok) = sess.opts.emit {
        return Ok(CompileOutput::Tokens(stream));
    }

    let unit = parse(stream).map_err(|diag| vec!(diag))?; // TOOD: Add vec of errors!

    if let Some(Stage::Ast) = sess.opts.emit {
        return Ok(CompileOutput::Ast(unit));
    }

    let resolved = resolve(&unit)?;

    // TODO: Add the rests
    Ok(CompileOutput::Ast(unit))
}

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

    let artifact = unwrap_or_emit_fatal(compile(&sess));

    match (sess.opts.emit, artifact) {
        (Some(Stage::Tok), CompileOutput::Tokens(stream)) => println!("{stream:?}"),
        (Some(Stage::Ast), CompileOutput::Ast(unit)) => println!("{unit:?}"),
        _ => {} // TODO: Add -o key
    }
}

pub fn entry() -> ExitCode {
    catch_fatal_error_marker(run)
}
