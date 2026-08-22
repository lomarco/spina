use std::{
    fs::File,
    io::{
        BufReader,
        Read
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
    },
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
