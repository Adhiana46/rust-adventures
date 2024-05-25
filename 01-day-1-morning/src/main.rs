fn main() {
    hello_world();
    variables();
    values();
    println!("Arithmetic: {}", arithmetic(10, 20, 30));
    type_inference()
}

fn hello_world() {
    println!("Hello, World!");
}

fn variables() {
    // by default variable are immutable
    let x: i32 = 32;

    // to make it mutable add mut keyword
    let mut y: i32 = 10;
    y = y * x;

    // unused variable show warning
    let z: i32;

    // to supress warning use _
    let _xx: i32;

    // println!("x: {}", x);
    println!("x: {x}");
    println!("y: {y}");
}

fn values() {
    // signed integer
    // i8 i16 i32 i64 i128 isize
    // literal: -10, 0, 1_000, 123_i64
    let i8_x: i8 = 127;
    println!("Signed integer 8: {}", i8_x);

    // unsigned integer
    // u8 u16 u32 u64 u128 usize
    // literal: 0, 123, 10_u16
    let u8_x: u8 = 255;
    println!("Unsigned integer 8: {}", u8_x);

    // literal unsigned integer 64
    let u64_literal = 8192_u64;
    println!("Literal Unsigned Integer 64: {}", u64_literal);

    // floating point
    // f32 f64
    // literal: 3.14, -10.0e20, 2_f32
    let f_32: f32 = 3.14;
    println!("Floating point 32: {}", f_32);

    let f_64: f64 = 2323.2324;
    println!("Floating point 64: {}", f_64);

    let f_literal = 23.55_f64;
    println!("Floating point 64 literal: {}", f_literal);

    // Unicode scalar values
    // char
    // 'a', 'α', '∞'
    let infiniti: char = '∞';
    println!("Char Infinity: {}", infiniti);

    let alpha: char = 'α';
    println!("Char Alpha: {}", alpha);

    let emoji: char = '🙂';
    println!("Char Emoji: {}", emoji);

    let emoji_literal = '⚡';
    println!("Char Literal: {}", emoji_literal);

    // Booleans
    // bool
    // true, false
    let is_gay: bool = false;
    println!("Are you gay? {}", is_gay);

    // literal
    let is_beauty = true;
    println!("Are you beauty? {}", is_beauty);
}

fn arithmetic(a: i32, b: i32, c: i32) -> i32 {
    return (a * b + b * c + c * a - a - b - c) / 1;
}

// Rust will look at how the variable is used to determine the type:
fn type_inference() {
    let x = 64; // u8
    let y = 2048; // i32

    fn f_u8(v: u8) {
        println!("f_u8: {}", v);
    }

    fn f_i32(v: i32) {
        println!("f_i32: {}", v);
    }

    f_u8(x);
    f_i32(y);
}
