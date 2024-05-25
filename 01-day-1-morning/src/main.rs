fn main() {
    println!("=== BASIC ===");
    hello_world();
    println!("\n");

    // Types & Values
    println!("=== Types & Values ===");
    variables();
    values();
    println!("Arithmetic: {}", arithmetic(10, 20, 30));
    type_inference();
    println!("Exercise Fibonacci: {}", exercise_fibonacci(20));
    println!("\n");

    // Control Flow Basics
    println!("=== Control Flow Basics ===");
    if_expressions();
    ternary_expressions();
    loops_while();
    loops_for();
    loops_loop();
    break_and_continue();
    break_label();
    blocks();
    shadowing();
    println!("\n");
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

// Exercise: Fibonacci
fn exercise_fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    return exercise_fibonacci(n - 1) + exercise_fibonacci(n - 2);
}

// if expression
fn if_expressions() {
    let x = 10;

    if x == 0 {
        println!("Zero");
    } else if x < 10 {
        println!("Biggish");
    } else {
        println!("Huge");
    }
}

// ternary operation, in other lang wkwkwk
fn ternary_expressions() {
    let x = if "wkwkwkwk" == "lol" { 0 } else { 100 };

    println!("ternary_expressions (wkwkwkwk == lol ? 0 : 100): {}", x);
}

fn loops_while() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("loops_while: {}", x);
}

fn loops_for() {
    // For loop with range (0 to 4)
    for x in 0..5 {
        println!("for range: {}", x);
    }

    // For loop with inclusive range (0 to 5)
    for x in 0..=5 {
        println!("for inclusive range: {}", x);
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("for elem: {}", elem);
    }
}

// The loop statement just loops forever, until a break
fn loops_loop() {
    let mut i = 0;
    loop {
        i += 1;
        println!("loop: {}", i);
        if i == 3 {
            break;
        }
    }
}

fn break_and_continue() {
    let mut i = 0;
    loop {
        i += 1;

        if i == 3 {
            // immediately start to next iteration
            continue;
        }

        if i > 5 {
            break;
        }

        println!("break_and_continue: {}", i);
    }
}

fn break_label() {
    let mut i = 0;
    'outerloop: loop {
        i += 1;

        for j in 0..=10 {
            println!("break_label => i: {} <--> j: {}", i, j);

            if j * i > 10 {
                break 'outerloop;
            }
        }

        if i > 100 {
            break;
        }
    }
}

fn blocks() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
}

fn shadowing() {
    let x = 5; // First declaration of x
    println!("First x: {}", x); // 5
    assert_eq!(x, 5);

    let x = x + 1; // shadowing first x
    println!("Shadowing First x: {}", x); // 6
    assert_eq!(x, 6);

    {
        let x = x * 2; // Shadowing second x
        println!("Shadowing Second x: {}", x); // 12
        assert_eq!(x, 12);

        let x = true;
        assert_eq!(x, true);
    }

    println!("Showing value of Second x: {}", x);
    assert_eq!(x, 6);
}
