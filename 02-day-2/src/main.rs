fn main() {
    // References
    println!("=== References ===");
    shared_references();
    mutual_borrow();
    immutual_borrow();
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
fn mutual_borrow() {
    let mut a = 'A';
    println!("mutual_borrow.a before: {}", a);
    let borrow_a: &mut char = &mut a;
    *borrow_a = 'C';
    println!("mutual_borrow.a after: {}", a);

    // if uncomment, will result in compilation error
    // because when passing a to println!, immutual borrow occurs
    // *borrow_a = 'D';
}

// When you borrow a value immutably, you cannot modify it. Multiple immutable borrows are allowed simultaneously.
fn immutual_borrow() {
    let mut a: char = 'A';
    let borrow_a: &char = &a;

    println!("immutual_borrow.a before: {}", a);
    println!("immutual_borrow.borrow_a before: {}", borrow_a);

    // change value of a
    // can't do that because it is borrowed by borrow_a
    // a = 'C';

    // can't do this because it is immutual borrow
    // *borrow_a = 'C';

    // it will success, because it can have many immutual borrow
    println!("immutual_borrow.a after: {}", a);
    println!("immutual_borrow.borrow_a after: {}", borrow_a);
}
