use std::path::PathBuf;

pub enum Input {
    File(PathBuf),
    Str(String),
}

#[derive(Debug)]
pub enum Stage {
    Tok,
    Ast,
}

pub struct Options {
    pub input: Input,
    pub emit: Option<Stage>,
    pub output: Option<PathBuf>,
}

pub struct Session {
    pub opts: Options,
}

impl Session {
    pub fn new(opts: Options) -> Self {
        Self { opts }
    }
}
