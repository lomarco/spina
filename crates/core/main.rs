use anyhow::{Context, Result}; // TODO: Delete it
use session::{Session, ParseSess, build_session};
use parser::parse;

use std::{
    env, // FIXME: Replace this code to clap
    fs::read_to_string,
    process::ExitCode,
};

fn run() -> Result<(), String> {
    // FIXME: Replace this code to clap
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {}: <filename> ", args[0]);
    }
    let filename = &args[1];
    // FIXME: Replace this code to clap

    let sess = build_session();
    let mut unit = parse(filename); // TODO: Move all file opening logic to parse_from_file

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e:?}");
            ExitCode::FAILURE
        }
    }
}
