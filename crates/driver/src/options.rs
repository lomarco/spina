use crate::cli;
use sess::Stage;

impl From<cli::Stage> for Stage {
    fn from(a: cli::Stage) -> Self {
        match a {
            cli::Stage::Tok => Stage::Tok,
            cli::Stage::Ast => Stage::Ast,
        }
    }
}
