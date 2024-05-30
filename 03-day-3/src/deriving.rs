#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

#[test]
fn derive_test() {
    let p1 = Player::default(); // use derive default
    println!("Default: {:?}", p1);

    // clone
    let mut p2 = p1.clone();
    p2.name = String::from("Yoona28");
    println!("P1: {:?}", p1);
    println!("P2: {:?}", p2);
}
