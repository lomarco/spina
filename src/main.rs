const MAGIC: u32 = 0x5350494e;

struct Header {
    magic: u32,
    flags: u16,
    constant_pool_count: u32,
    constant_pool_offset: u32,
    functions_count: u32,
    functions_offset: u32
}
fn main() {
    println!("Hello, spina!");
}
