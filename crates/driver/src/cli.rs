use clap::{Args, Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "spinac", version, about, long_about = None, arg_required_else_help = true)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub input: InputArgs,

    #[arg(long, value_enum, value_name = "STAGE")]
    pub emit: Option<Stage>,

    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub(crate) struct InputArgs {
    pub file: Option<PathBuf>,

    #[arg(short = 'e', long, value_name = "CODE")]
    pub eval: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum Stage {
    #[value(alias = "tokens")]
    Tok,
    Ast,
    // TODO: Add the rest stages.
}
