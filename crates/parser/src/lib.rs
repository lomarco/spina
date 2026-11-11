mod expr;
mod item;
mod parser;
mod stmt;
mod ty;

#[cfg(test)]
mod tests;

use ast::Unit;
use err::{Diag, raise};
use lex::flex;
use parser::Parser;
use sess::{Input, Session};
use std::{fs::read_to_string, io::ErrorKind, path::Path};

fn unwrap_or_emit_fatal<T>(expr: Result<T, Vec<Diag>>) -> T {
    expr.unwrap_or_else(|errs| {
        for err in errs {
            err.emit();
        }
        raise()
    })
}

fn new_parser_from_file(path: &Path) -> Result<Parser, Vec<Diag>> {
    let cont = read_to_string(path).unwrap_or_else(|e| {
        // TODO: Maybe delete unwrap_or_else
        let msg = match e.kind() {
            ErrorKind::NotFound => format!("couldn't find file `{}`", path.display()),
            ErrorKind::PermissionDenied => {
                format!("permission denied when opening file `{}`", path.display())
            }
            ErrorKind::IsADirectory => format!("`{}` is a directory", path.display()),
            _ => format!("couldn't read `{}`: {}", path.display(), e),
        };

        Diag::fatal(msg)
    });

    let stream = flex(&cont)?;

    let parser = Parser::new(stream);
    Ok(parser)
}

fn new_parser_from_str(str: &str) -> Result<Parser, Vec<Diag>> {
    Ok(Parser::new(flex(str)?))
}

pub fn parse(sess: &Session) -> Unit {
    // TODO: Add new_parser_from_source_str
    unwrap_or_emit_fatal(match &sess.input {
        Input::File(file) => new_parser_from_file(file),
        Input::Str(str) => new_parser_from_str(str),
    })
    .parse_unit()
    .unwrap_or_else(|parse_error| {
        parse_error.emit();
        raise()
    })
}
