use std::{
    fs::File,
    io::{
        BufReader,
        Read
    },
    env // FIXME: Replace this code to clap
};

const MAGIC: [u8; 4] = *b"SPIN";

struct Flags(u16);

impl Flags {
    const OPTIMIZE: Self = Self(1 << 0);
    const SWAG: Self     = Self(1 << 1);

    fn from(bits: u16) -> Self {
        Self(bits)
    }
}

struct Header {
    magic: u32,
    flags: Flags,
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

    let mut flags = [0u8; 2];
    reader.read_exact(&mut flags)?;

    let mut constant_pool_count = [0u8; 4];
    reader.read_exact(&mut constant_pool_count)?;

    let mut constant_pool_offset = [0u8; 4];
    reader.read_exact(&mut constant_pool_offset)?;

    let mut functions_count = [0u8; 4];
    reader.read_exact(&mut functions_count)?;

    let mut function_offset = [0u8; 4];
    reader.read_exact(&mut function_offset)?;

    let mut hdr = Header {
        magic: u32::from_le_bytes(magic),
        flags: Flags::from(u16::from_le_bytes(flags)),
        constant_pool_count: u32::from_le_bytes(constant_pool_count),
        constant_pool_offset: u32::from_le_bytes(constant_pool_offset),
        functions_count: u32::from_le_bytes(functions_count),
        functions_offset: u32::from_le_bytes(function_offset),
    };

    Ok(())
}
