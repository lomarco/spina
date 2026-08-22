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

struct Spc {
    header: Header,
    constant: Constant,
    functions: Function,
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
