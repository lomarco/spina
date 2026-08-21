use std::{
    fs::File,
    io::{
        BufReader,
        Read
    },
    env // FIXME: Replace this code to clap
};

const MAGIC: u32 = 0x5350494e;

struct Flags(u16);

impl Flags {
    const OPTIMIZE: Self = Self(1 << 0);
    const DEBUG: Self     = Self(1 << 1);

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

enum Tags {
    Int,
    Float,
    IntRef,
    FloatRef,
    FuncRef
}

enum Data {
    Int(i32),
    Float(f32),
    IntRef(u32),
    FloatRef(u32),
    FuncRef(u32)
}

struct Constant {
    tag: Tags,
    data_len: u32,
    data: Data
}

struct Function {
    name_index: u32,
    param_count: u16,
    locals_count: u16,
    code: Vec<u8>
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

    let mut flags = [0u8; 2];
    reader.read_exact(&mut flags)?;

    let mut constant_pool_count = [0u8; 4];
    reader.read_exact(&mut constant_pool_count)?;

    let mut constant_pool_offset = [0u8; 4];
    reader.read_exact(&mut constant_pool_offset)?;

    let mut functions_count = [0u8; 4];
    reader.read_exact(&mut functions_count)?;

    let mut functions_offset = [0u8; 4];
    reader.read_exact(&mut functions_offset)?;

    let mut hdr = Header {
        magic: u32::from_le_bytes(magic),
        flags: Flags::from(u16::from_le_bytes(flags)),
        constant_pool_count: u32::from_le_bytes(constant_pool_count),
        constant_pool_offset: u32::from_le_bytes(constant_pool_offset),
        functions_count: u32::from_le_bytes(functions_count),
        functions_offset: u32::from_le_bytes(functions_offset),
    };

    Ok(())
}
