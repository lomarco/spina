use anyhow::{Context, Result}; // TODO: Delete it
use lex::flex;

use std::{
    env, // FIXME: Replace this code to clap
    fs::read_to_string,
    process::ExitCode,
};

fn run() -> Result<()> {
    // FIXME: Replace this code to clap
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {}: <filename> ", args[0]);
    }
    let filename = &args[1];
    // FIXME: Replace this code to clap

    let cont =
        read_to_string(filename).with_context(|| format!("failed to read file '{}'", filename))?;
    match flex(&cont) {
        Ok(tokens) => println!("{tokens:?}"),
        Err(err) => return Err(err.into()),
    }

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
