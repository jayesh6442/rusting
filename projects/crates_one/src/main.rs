mod inventory;
mod order;
fn main() {


    let mut s = String::from("");
    println!("Hello, world!");
    inventory::testing();
    println!("{}", order::MAN);
    println!("{}", inventory::products::SUB_MODULE);
}
