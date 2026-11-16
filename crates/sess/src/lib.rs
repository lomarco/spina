use std::path::PathBuf;

#[derive(Debug)]
pub enum Stage {
    Tok,
    Ast,
}

pub struct Options {
    pub emit: Option<Stage>,
    pub output: Option<PathBuf>,
}

pub struct Source {
    pub name: Option<PathBuf>,
    pub text: String,
}

pub struct Session {
    pub opts: Options,
    pub source: Source,
}

impl Session {
    pub fn new(opts: Options, source: Source) -> Self {
        Self { opts, source }
    }
}
