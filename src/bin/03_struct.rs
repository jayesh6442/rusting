#[derive(Debug)]
struct Jayesh {
    name: String,
    age: i32,
    is_married: bool,
    education: String,
}

impl Jayesh {
    fn display(&self) {
        println!("{}", self.name);
    }
    fn update_name(&mut self, name: String) {
        self.name = name;
    }
    fn new(name: String, age: i32, is_married: bool, education: String) -> Self {
        Self {
            name,
            age,
            is_married,
            education,
        }
    }
}
struct TupalType(i64, i64);
struct Logger;

impl Logger {
    fn log(msg: &str) {
        println!("[log]: {}", msg);
    }
}
trait Log {
    fn log(&self, msg: &str);
}
impl Log for Logger {
    fn log(&self, msg: &str) {
        println!("[msg]: {}", msg);
    }
}
fn main() {
    let jayesh: Jayesh = Jayesh {
        name: String::from("jayesh"),
        age: 23,
        is_married: false,
        education: String::from("BE"),
    };

    println!(
        "the age of {} is {} and hi is {} and his education is {}",
        jayesh.age, jayesh.name, jayesh.is_married, jayesh.education
    );
    let mut user_two = jayesh;
    println!("{:?}", user_two);
    // println!("{:?}", jayesh);
    user_two.display();
    user_two.update_name(String::from("dev jayesh"));
    user_two.display();

    let user_three: Jayesh = Jayesh {
        name: (String::from("aaatu jatu")),
        age: (32),
        is_married: (false),
        education: (String::from("God of coding")),
    };

    user_three.display();

    let test_tupe: TupalType = TupalType(234, 232);
    println!("{} {}", test_tupe.0, test_tupe.1);

    let person = Jayesh::new(
        String::from("jayesh is cook"),
        23,
        false,
        String::from("he is literally god"),
    );

    person.display();
    // let logger = Logger;
    Logger::log("hi there this is log");
    let loging = Logger;
    loging.log("hi there this is second logger")
}
