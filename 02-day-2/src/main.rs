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
