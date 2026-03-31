#[derive(Debug)]
struct Coffe {
    price: i32,
    name: String,
    is_hot: bool,
}

impl Coffe {
    fn display(&self) {
        println!(
            "this is {} the price of it  is {} and its  {}",
            self.name, self.price, self.is_hot
        );
    }
}

impl Coffe {
    fn new(price: i32, name: String, is_hot: bool) -> Self {
        Coffe {
            price,
            name,
            is_hot,
        }
    }
}
fn main() {
    println!("hi there this is second part of the  sturct");
    let mocha = Coffe::new(230, String::from("mocha"), true);
    mocha.display();

    let array_of_coffe: [Coffe; 4] = [
        Coffe::new(239, String::from("test coffe"), false),
        Coffe::new(239, String::from("test coffe"), false),
        Coffe::new(239, String::from("test coffe"), false),
        Coffe::new(239, String::from("test coffe"), false),
    ];

    for i in array_of_coffe {
        println!("{:?}", i);
    }
    let one_of = Coffe { ..mocha };
    one_of.display();
}
