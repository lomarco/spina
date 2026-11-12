use crate::cli::{self, Cli, InputArgs};
use sess::{Input, Options, Stage};

impl From<cli::Stage> for Stage {
    fn from(a: cli::Stage) -> Self {
        match a {
            cli::Stage::Tok => Stage::Tok,
            cli::Stage::Ast => Stage::Ast,
        }
    }
}

impl From<InputArgs> for Input {
    fn from(a: InputArgs) -> Self {
        match (a.file, a.eval) {
            (Some(path), None) => Input::File(path),
            (None, Some(code)) => Input::Str(code),
            _ => unreachable!(),
        }
    }
}

impl From<Cli> for Options {
    fn from(a: Cli) -> Self {
        Options {
            input: a.input.into(),
            emit: a.emit.map(Stage::from),
            output: a.output,
        }
    }
}
