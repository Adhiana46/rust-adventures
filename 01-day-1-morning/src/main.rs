fn main() {
    hello_world();
    variables();
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
