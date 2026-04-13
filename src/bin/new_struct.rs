struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn create(name: String, age: i32) -> Self {
        Self { name, age }
    }
}

fn main() {
    let person = Person::create("John".to_string(), 30);

    let one: String = "one".to_string();
    println!("{}", one);
    println!("Name: {}, Age: {}", person.name, person.age);
}
