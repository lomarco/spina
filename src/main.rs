use std::{
    fs::File,
    io::{
        BufReader,
        Read
    },
    env // FIXME: Replace this code to clap
};

const MAGIC: [u8; 4] = *b"SPIN";

struct Header {
    magic: u32,
    flags: u16,
    constant_pool_count: u32,
    constant_pool_offset: u32,
    functions_count: u32,
    functions_offset: u32
}

fn main() -> std::io::Result<()> {
    // FIXME: Replace this code to clap
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {}: <filename> ", args[0]);
    }
    let filename = &args[1];
    // FIXME: Replace this code to clap

    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;

    if magic != MAGIC {
        println!("magic fault: '{}'", std::str::from_utf8(&magic).expect("Magic number fault"));
    }

    Ok(())
}
