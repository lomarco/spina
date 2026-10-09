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

impl Session { // TODO: Delete this excess struct
}

pub struct ParseSess { // TODO: Delete this excess struct
    // TODO: Add the rest fields
}
