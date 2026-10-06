use anyhow::{Context, Result}; // TODO: Delete it
use session::{Session, ParseSess, build_session};
use parser::parse;
use std::panic::catch_unwind;
use errors::FatalErrorMarker;

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
}

fn main() -> ExitCode {
    match catch_unwind(run) {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
