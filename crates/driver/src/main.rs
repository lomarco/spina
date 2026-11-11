mod cli;
mod options;

use err::catch_fatal_error_marker;
use parser::parse;
use sess::Session;
use std::process::ExitCode;
use cli::Args;
use clap::Parser;

fn run() {
    let sess = Session::new(Args::parse());
    let unit = parse(&sess); // TODO: Move all file opening logic to parse_from_file

    println!("{unit:#?}"); // FIXME
}

fn main() -> ExitCode {
    catch_fatal_error_marker(run)
}
