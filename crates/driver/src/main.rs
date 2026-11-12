mod cli;
mod options;

use clap::Parser;
use cli::Cli;
use err::catch_fatal_error_marker;
use parser::parse;
use sess::Session;
use std::process::ExitCode;

fn run() {
    let sess = Session::new(Cli::parse().into());
    let unit = parse(&sess);
}

fn main() -> ExitCode {
    catch_fatal_error_marker(run)
}
