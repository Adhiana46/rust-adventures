mod associated_types;
mod deriving;
mod methods;
mod pattern_matching;
mod traits;

use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    mood: Mood,
}

enum Mood {
    Happy,
    Normal,
    Depressed,
}

// implement custom fmt::Debug
impl fmt::Debug for Mood {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mood::Happy => write!(f, "😊"),
            Mood::Normal => write!(f, "😐"),
            Mood::Depressed => write!(f, "😔"),
        }
    }
}

fn create_person(name: String, age: u8) -> Person {
    let p = Person {
        name: name,
        age: age,
        mood: Mood::Normal,
    };

    return p;
}

fn print_person(p: &Person) {
    println!(
        "Hello my name is {}, my age is {} {:?}",
        p.name, p.age, p.mood
    );
}

fn make_person_happy(p: &mut Person) {
    p.mood = Mood::Happy;
}

fn make_person_depressed(p: &mut Person) {
    p.mood = Mood::Depressed;
}

fn add_age(p: &mut Person, inc: u8) {
    p.age = p.age + inc;
}

fn main() {
    let mut adhiana: Person = create_person(String::from("Adhiana Mastur"), 18);
    // let mut adhiana: Person = Person {
    //     name: String::from("Adhiana Mastur"),
    //     age: 26,
    //     mood: Mood::Happy,
    // };

    make_person_happy(&mut adhiana);
    print_person(&adhiana);
    println!("Eight years later....");
    add_age(&mut adhiana, 8);
    make_person_depressed(&mut adhiana);
    print_person(&adhiana);
    // println!("Person: {:?}", adhiana);
}
