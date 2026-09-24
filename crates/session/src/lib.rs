use std::path::PathBuf;

pub enum Input {
    File(PathBuf),
    Str(String),
}

pub struct Session {
    pub psess: ParseSess,
    pub input: Input,
    // TODO: Add the rest field(f.e code_stats or timings)
}

// pub fn build_session() -> Session {
//    ;
//}

impl Session {
}

pub struct Source (String);

pub struct ParseSess {
    pub source: Source,
    // TODO: Add the rest fields
}
