use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret);

    loop {
        let mut input = String::from("");
        io::stdin()
            .read_line(&mut input)
            .expect("error reading user input");
        // println!("{}", input);

        let user_no: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // println!("{}", user_no);
        match user_no.cmp(&secret) {
            Ordering::Less => println!("guess is to less"),
            Ordering::Greater => println!("you guess to high"),
            Ordering::Equal =>{
                println!("number matched");
                break;

            }
        }
    }
}
