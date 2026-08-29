use std::env; // FIXME: Replace this code to clap
use lex::flex;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // FIXME: Replace this code to clap
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {}: <filename> ", args[0]);
    }
    let filename = &args[1];
    // FIXME: Replace this code to clap

    let mut cont = String::new();
    File::open(filename)?.read_to_string(&mut cont)?;
    match flex(&cont) {
        Ok(tokens) => println!("{tokens:?}"),
        Err(err) => println!("{err}")
    }

    Ok(())
}
