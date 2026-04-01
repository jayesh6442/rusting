// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
// }
struct User {
    active: bool,
    username: String,
    email: String,
}
fn main() {
    let user_name = String::from("jayesh12");
    let user_emain = String::from("jayesh12@gmail.com`");

    let user_one = User {
        active: false,
        username: user_name,
        email: user_emain,
    };

    println!("{}", user_one.email);
    println!("{}", user_one.active);
    println!("{}", user_one.username);
}
