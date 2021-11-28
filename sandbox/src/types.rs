/*
    Primitive Types:
    Integers : u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
    Floats : f32, f64
    Boolean: true, false
    Chars
    Tuples
    Arrays

    Rust is statically typed language.
*/

pub fn run() {
    // integers
    println!("Max size : i32 = {}", std::i32::MAX);
    // println!("Max size : f64 = {}", std::f64::MAX);

    // bool
    let is_active = true;
    println!("{:?}", is_active);

    // char
    let a1 = '\u{1F600}';
    println!("{}", a1);
}
