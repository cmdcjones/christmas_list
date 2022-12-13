use std::io;
use std::fmt;

struct Person {
    name: String,
    received: bool,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is {}.\n", self.name)?;
        if self.received == false {
            write!(f, "They have not received a gift.")
        } else {
            write!(f, "They have received a gift.")
        }
    }
}

fn create_person(name: String) -> Person {
    Person {
        name,
        received: false,
    }
}

fn main() {
    println!("Enter a name:");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Could not read in name");
    let name: String = name.trim().to_string();

    let person: Person = create_person(name);
    // let people: vec!<Person> = vec!

    println!("{person}");
}
