use std::collections::HashMap;

trait Testtrait {
    fn get_details(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}
#[derive(Debug)]
struct Hotel {
    name: String,
    res: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: String) -> Self {
        Self {
            name,
            res: HashMap::new(),
        }
    }
}

impl Testtrait for Hotel {
    fn get_details(&self) -> String {
        format!("{} is best hotel", self.name)
    }
    fn book(&mut self, name: &str, nights: u32) {
        self.res.insert(name.to_string(), nights);
    }
}

fn main() {
    println!("traits in rust")
}
