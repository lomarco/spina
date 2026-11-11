use std::path::PathBuf;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, propagate_version = true, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Build(BuildArgs),
}

#[derive(Args)]
pub struct BuildArgs {
    #[command(flatten)]
    pub input: InputArgs,

    #[arg(long, value_enum, value_name = "STAGE")]
    pub emit: Option<Stage>,

    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct InputArgs {
    pub file: Option<PathBuf>,

    #[arg(short = 'e', long, value_name = "CODE")]
    pub eval: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Stage {
    #[value(alias = "tokens")]
    Tok,
    Ast,
    // TODO: Add the rest stages.
}
