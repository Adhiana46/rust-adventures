mod pattern_matching;

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

fn print_person(p: &Person) {
    println!(
        "Hello my name is {}, my age is {} {:?}",
        p.name, p.age, p.mood
    );
}

fn make_person_depressed(p: &mut Person) {
    p.mood = Mood::Depressed;
}

fn main() {
    let mut adhiana: Person = Person {
        name: String::from("Adhiana Mastur"),
        age: 26,
        mood: Mood::Happy,
    };

    print_person(&adhiana);
    make_person_depressed(&mut adhiana);
    print_person(&adhiana);
    // println!("Person: {:?}", adhiana);
}
