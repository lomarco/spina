use std::path::PathBuf;

pub enum Input {
    File(PathBuf),
    Str(String),
}

pub struct Session {
    pub input: Input,
    // TODO: Add the rest field(f.e code_stats or timings)
}
