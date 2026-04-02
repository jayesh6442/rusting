struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn display(self) {
        println!("{} {}", self.age, self.name);
    }
}

impl Person {
    fn display_two(&self) {
        println!("{} {}", self.age, self.name);
    }
}

impl Person {
    fn create(name: String, age: i32) -> Self {
        Self { name, age }
    }
}
fn main() {
    let person = Person {
        name: String::from("jayesh"),
        age: 23,
    };

    person.display_two();
    person.display();
    let p2 = Person::create(String::from("person two"), 23);
    p2.display();
}
