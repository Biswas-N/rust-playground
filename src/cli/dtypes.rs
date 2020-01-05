/*
Primitive Types -- (Along with num of bits occupied)
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn run() {
    println!("----> Data Types : See CLI/dtypes.rs <----");

    // Implicit Type
    // Default is "i32"
    let _x = 1;

    // Default is "f64"
    let _y = 2.5;

    // Explicit type
    let _z: i64 = 101010;

    // Find Max Size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    let is_equal = _x == _z;

    // Char and Unicode
    let a1 = 'A';
    let face = '\u{1F60D}';

    println!("{:?}", (_x, _y, _z, is_active, is_equal, a1, face));
}
