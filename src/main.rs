use std::{
    fs::File,
    io::{
        BufReader,
        Read
    }
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
    let file = File::open("bc.spc")?;
    let mut reader = BufReader::new(file);

    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;

    if magic == MAGIC {
        println!("magic number:");
        println!("{}", std::str::from_utf8(&magic).expect("Magic number fault"));
        println!("{:#04x?}", magic);
    } else {
        println!("magic fault!!!");
    }

    Ok(())
}
