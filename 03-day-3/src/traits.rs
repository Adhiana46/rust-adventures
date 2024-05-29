trait Pet {
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

struct Cat {
    name: String,
    age: u8,
}

impl Pet for Cat {
    fn talk(&self) -> String {
        format!("Meow, my name is {}!", self.name)
    }
}

#[test]
fn implement_traits_test() {
    let lady = Cat {
        name: String::from("Lady"),
        age: 2,
    };
    lady.greet();
}
