use err::catch_fatal_error_marker;
use parser::parse;
use sess::Session;

use std::{
    env::args, // FIXME: Replace this code to clap
    process::ExitCode,
};

fn run() {
    // FIXME: Replace this code to clap
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("usage: {}: <filename> ", args[0]);
    }
    let filename = &args[1];
    // FIXME: Replace this code to clap

    let sess = Session::new(filename);
    let unit = parse(&sess); // TODO: Move all file opening logic to parse_from_file

    println!("{unit:#?}"); // FIXME
}

fn main() -> ExitCode {
    catch_fatal_error_marker(run)
}
