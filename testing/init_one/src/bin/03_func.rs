use std::any::type_name_of_val;

fn main() {
    println!("hi there we are in the functions");
    let no = 3;
    let res = if no % 2 == 1 { "odd" } else { "even" };
    println!("{}", res);
    println!("{}", type_name_of_val(&res));
    println!("{}", type_name_of_val(&no));
}
