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

    fn change_type(mut self: Self) {
        self.is_hot = !self.is_hot;
        self.display();
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
    let mut one_of = Coffe { ..mocha };
    one_of.display();
    update_name(&mut one_of);
    one_of.display();
    test_fn(&one_of);
    one_of.change_type();
    one_of.display();
}

//? mut and unmut fn on structs

fn update_name(coffe: &mut Coffe) {
    coffe.name = String::from("updated name");
}

fn test_fn(coffe: &Coffe) {
    println!("{}  {} {}", coffe.name, coffe.price, coffe.is_hot);
}
