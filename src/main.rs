use std::io;
use std::fmt;

struct Person {
    name: String,
    received: bool,
}

// More user friendly printing for Person
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

// builder function
fn create_person(name: String) -> Person {
    Person {
        name,
        received: false,
    }
}

fn main() {
    println!("Enter a name:");

    let mut name = String::new();
    // read input from stdin
    io::stdin()
        .read_line(&mut name)
        .expect("Could not read in name");
    // change &str to String
    let name: String = name.trim().to_string();

    // person will mutate in the future
    let mut person: Person = create_person(name);
    // time to read up on vectors!
    // let people: vec!<Person> = vec!

    println!("{person}");
}
