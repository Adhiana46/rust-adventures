fn main() {
    // References
    println!("=== References ===");
    shared_references();
    mutable_borrow();
    immutable_borrow();
    borrow_slice();
    strings();
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
