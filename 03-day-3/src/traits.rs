trait Animal {
    fn leg_count(&self) -> u32;
}

trait Pet: Animal {
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

struct Cat {
    name: String,
    age: u8,
}

impl Animal for Cat {
    fn leg_count(&self) -> u32 {
        4
    }
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
    println!("I've {} legs", lady.leg_count());
}
