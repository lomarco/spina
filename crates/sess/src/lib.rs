use std::path::PathBuf;

pub enum Input {
    File(PathBuf),
    Str(String),
}

pub struct Session {
    pub input: Input,
}

impl Session {
    pub fn new(path: &str) -> Self {
        Self {
            input: Input::File(PathBuf::from(path)),
        }
    }
}
