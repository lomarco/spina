mod cli;

use clap::Parser;
use cli::Cli;
use err::catch_fatal_error_marker;
use parser::parse;
use sess::Session;
use std::process::ExitCode;

fn run() {
    let sess = Session::new(Cli::parse());
    let unit = parse(&sess); // TODO: Move all file opening logic to parse_from_file
}

fn main() -> ExitCode {
    catch_fatal_error_marker(run)
}
