use std::{
    io::{
        // BufReader,
        Read,
        Error // TODO: Add error.rs
    },
};

const MAGIC: u32 = 0x5350494e;

struct Flags(u16);

impl Flags {
    const OPTIMIZE: Self = Self(1 << 0);
    const DEBUG: Self    = Self(1 << 1);
    const NONE: Self     = Self(0);

    fn new() -> Self {
        Self::NONE
    }

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
    functions_offset: u32 // FIXME: Maybe add timestamp
}

impl Header {
    fn new() -> Self {
        Self {
            magic: 0,
            flags: Flags::new(),
            constant_pool_count: 0,
            constant_pool_offset: 0,
            functions_count: 0,
            functions_offset: 0
        }
    }
    fn decode(&mut self, r: &mut impl Read) -> Result<(), Error> { // TODO: Add error.rs for magic number
                                                          // checking and the rest fields of Header.
        let mut magic = [0u8; 4];
        r.read_exact(&mut magic)?;

        let mut flags = [0u8; 2];
        r.read_exact(&mut flags)?;

        let mut constant_pool_count = [0u8; 4];
        r.read_exact(&mut constant_pool_count)?;

        let mut constant_pool_offset = [0u8; 4];
        r.read_exact(&mut constant_pool_offset)?;

        let mut functions_count = [0u8; 4];
        r.read_exact(&mut functions_count)?;

        let mut functions_offset = [0u8; 4];
        r.read_exact(&mut functions_offset)?;

        self.magic = u32::from_le_bytes(magic);
        self.flags = Flags::from(u16::from_le_bytes(flags));
        self.constant_pool_count = u32::from_le_bytes(constant_pool_count);
        self.constant_pool_offset = u32::from_le_bytes(constant_pool_offset);
        self.functions_count = u32::from_le_bytes(functions_count);
        self.functions_offset = u32::from_le_bytes(functions_offset);
        Ok(())
    }
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

struct Constant { // TODO: Add soa
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

struct Spc {
    header: Header,
    constant: Vec<Constant>,
    functions: Vec<Function>,
}

impl Spc {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            constant: Vec::new(),
            functions: Vec::new()
        }
    }
    pub fn decode_from(&mut self, r: &mut impl Read) -> Result <(), Error> {
        self.header.decode(r)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
