use std::io;
fn main() {
    println!("hi there we are in the control flow of rust cofing");

    // let age = 23;

    let mut marks: String = String::from("");

    println!("Enter the marks:");
    io::stdin()
        .read_line(&mut marks)
        .expect("Fail to read the user input");

    let marks: i32 = marks
        .trim()
        .parse()
        .expect("invalid input pls enter the number only");

    if marks >= 90 {
        println!("you scored grade A");
    } else if marks >= 80 {
        println!("you scored grade B");
    } else if marks >= 70 && marks <= 80 {
        println!("you scored grade C")
    } else {
        println!("you faied man you have to try better");
    }
}
