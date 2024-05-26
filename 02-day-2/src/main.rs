fn main() {
    // References
    println!("=== References ===");
    shared_references();
    mutable_borrow();
    immutable_borrow();
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
