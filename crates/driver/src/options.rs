use crate::cli;
use err::Diag;
use sess::{Source, Stage};
use std::{fs::read_to_string, io::ErrorKind};

impl From<cli::Stage> for Stage {
    fn from(a: cli::Stage) -> Self {
        match a {
            cli::Stage::Tok => Stage::Tok,
            cli::Stage::Ast => Stage::Ast,
        }
    }
}

impl From<cli::InputArgs> for Source {
    fn from(input: cli::InputArgs) -> Self {
        match (input.file, input.eval) {
            (Some(path), None) => {
                let text = read_to_string(&path).unwrap_or_else(|e| {
                    Diag::fatal(match e.kind() {
                        ErrorKind::NotFound => format!("couldn't find file `{}`", path.display()),
                        ErrorKind::PermissionDenied => {
                            format!("permission denied when opening file `{}`", path.display())
                        }
                        ErrorKind::IsADirectory => format!("`{}` is a directory", path.display()),
                        _ => format!("couldn't read `{}`: {}", path.display(), e),
                    })
                });
                Source {
                    name: Some(path),
                    text,
                }
            }
            (None, Some(code)) => Source {
                name: None,
                text: code,
            },
            _ => unreachable!("clap enforces exactly one of `-e` / input file"),
        }
    }
}
