use session::{Session, ParseSess, build_session};
use parser::parse;
use errors::catch_fatal_error_marker;

use std::{
    env, // FIXME: Replace this code to clap
    fs::read_to_string,
    process::ExitCode,
};

fn run() {
    // FIXME: Replace this code to clap
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {}: <filename> ", args[0]);
    }
    let filename = &args[1];
    // FIXME: Replace this code to clap

    let sess = build_session(filename);
    let unit = parse(sess); // TODO: Move all file opening logic to parse_from_file

    println!("{unit}"); // FIXME
}

fn main() -> ExitCode {
    catch_fatal_error_marker(run)
}
