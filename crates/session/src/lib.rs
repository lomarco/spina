use std::path::PathBuf;
use errors::{Diag, Level};

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
    pub diag: Diag
    // TODO: Add the rest fields
}

impl ParseSess {
    pub fn struct_fatal(self, msg: String) -> Diag {
        Diag::new(Level::Fatal, msg)
    }

    pub fn fatal(self, msg: String) -> ! {
        self.struct_fatal(msg).emit_fatal()
    }
}
