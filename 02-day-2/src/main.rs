// Static Variables
// Imutable Static
static GREETING: &str = "This is Rusty Journey";

// Mutable Static
static mut COUNTER: i64 = 0;

// Global Constants
const MAX_POINTS: u32 = 100_000;

// Module Constants
mod config {
    pub const BUFFER_SIZE: usize = 1024;
    pub const TIMEOUT: u32 = 30;
}

fn main() {
    // Constants within function
    const PI: f64 = 3.141592653589793;
    const DAYS_IN_WEEK: u32 = 7;

    // should add unsafe block to mutate mutable static
    unsafe {
        for i in 0..100 {
            COUNTER += 1;
        }
    }

    // References
    println!("=== References ===");
    shared_references();
    mutable_borrow();
    immutable_borrow();
    borrow_slice();
    strings();
    println!("Static Variable Imutable (Default): {}", GREETING);
    unsafe {
        println!("Static Variable Mutable: {}", COUNTER);
    }
    println!("Constants.Global: {}", MAX_POINTS);
    println!("Constants.Mod.Config.BUFFER_SIZE: {}", config::BUFFER_SIZE);
    println!("Constants.Mod.Config.TIMEOUT: {}", config::TIMEOUT);
    println!("Constants.Local.PI: {}", PI);
    println!("Constants.Local.DAYS_IN_WEEK: {}", DAYS_IN_WEEK);
    println!("\n");
}

// A reference provides a way to access another value without taking responsibility for the value, and is also called “borrowing”. Shared references are read-only, and the referenced data cannot change.
fn shared_references() {
    let a = 'A';
    let b = 'B';
    println!("a: {}", a);
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
}

// When you borrow a value mutably, you can modify it. However, you can only have one mutable borrow at a time to prevent data races.
fn mutable_borrow() {
    let mut a = 'A';
    println!("mutable_borrow.a before: {}", a);
    let borrow_a: &mut char = &mut a;
    *borrow_a = 'C';
    println!("mutable_borrow.a after: {}", a);

    // if uncomment, will result in compilation error
    // because when passing a to println!, immutable borrow occurs
    // *borrow_a = 'D';
}

// When you borrow a value immutably, you cannot modify it. Multiple immutable borrows are allowed simultaneously.
fn immutable_borrow() {
    let mut a: char = 'A';
    let borrow_a: &char = &a;

    println!("immutable_borrow.a before: {}", a);
    println!("immutable_borrow.borrow_a before: {}", borrow_a);

    // change value of a
    // can't do that because it is borrowed by borrow_a
    // a = 'C';

    // can't do this because it is immutable borrow
    // *borrow_a = 'C';

    // it will success, because it can have many immutable borrow
    println!("immutable_borrow.a after: {}", a);
    println!("immutable_borrow.borrow_a after: {}", borrow_a);
}

fn borrow_slice() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    // try to change a[3], within range of s
    // error: because it is borrowed
    // a[3] = 11;

    // try to change a[0], not in range of s
    // error: can't change because it borrowed
    // a[0] = 11;

    println!("s: {s:?}");
}

fn strings() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[s2.len() - s1.len()..];
    println!("s3: {s3}");
}

/// Calculate the magnitude of the given vector.
fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut sum_vector = 0.0;
    for v in vector {
        sum_vector += v * v;
    }

    return sum_vector.sqrt();
}

/// Change the magnitude of the vector to 1.0 without changing its direction.
fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    for item in vector {
        *item /= mag;
    }
}

#[test]
fn geometry_test() {
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}

// Named Struct
struct Person {
    name: String,
    age: u8,
}

fn describe_person(p: &Person) {
    println!("{} is {} years old", p.name, p.age);
}

#[test]
fn person_test() {
    let mut adhiana = Person {
        name: String::from("Adhiana"),
        age: 26,
    };
    describe_person(&adhiana);

    adhiana.age = 28;
    describe_person(&adhiana);

    let name = "Mastur".to_string();
    let age = 30;
    let mastur = Person { name, age };
    describe_person(&mastur);

    let john = Person {
        name: String::from("John"),
        ..mastur
    };
    describe_person(&john);
}

// Tuple Struct
struct Coordinate(f64, f64);

#[test]
fn coordinate_test() {
    let myhome = Coordinate(-7.795580, 110.369492);
    println!("My Home Coordinate: {}, {}", myhome.0, myhome.1);
}

// ENUM
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple Variant
    Run(Direction),              // Tuple Variant
    Teleport { x: u32, y: u32 }, // Struct Variant
}

#[derive(Debug)]
#[repr(u32)]
enum Bar {
    A, // 0
    B = 1000,
    C, // 1001
    D, // 1002
}

#[test]
fn enum_test() {
    let mut m: PlayerMove = PlayerMove::Pass;
    println!("Player Move: {:?}", m);

    m = PlayerMove::Run(Direction::Left);
    println!("Player Move: {:?}", m);

    m = PlayerMove::Teleport { x: 12, y: 55 };
    println!("Player Move: {:?}", m);

    println!("Bar Enum:");
    println!("- A => {:?}", Bar::A);
}
