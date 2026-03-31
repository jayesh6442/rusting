use std::io;

fn main() {
    println!("hi there we are in the control flow of rust cofing");

    // let age = 23;

    // let mut marks: String = String::from("");

    // println!("Enter the marks:");
    // io::stdin()
    //     .read_line(&mut marks)
    //     .expect("Fail to read the user input");

    // let marks: i32 = marks
    //     .trim()
    //     .parse()
    //     .expect("invalid input pls enter the number only");

    // if marks >= 90 {
    //     println!("you scored grade A");
    // } else if marks >= 80 {
    //     println!("you scored grade B");
    // } else if marks >= 70 && marks <= 80 {
    //     println!("you scored grade C")
    // } else {
    //     println!("you faied man you have to try better");
    // }

    // ? loops

    let name: String = String::from("jayesh");
    let last_name: String = String::from("Sarvaiya");
    let education: String = String::from("BE");
    let a: [String; 3] = [name, last_name, education];

    for i in a {
        println!("{}", i);
    }
    // ? above one is for of kind of loop

    // ! pure loop

    let mut b = 10;
    loop {
        println!("{}", b);
        if b == 0 {
            break;
        }
        b -= 1;
    }
    is_even_odd(5);

    let mut day: String = String::from("");

    io::stdin().read_line(&mut day).expect("error reading ");
    let day = day.trim();
    // let sunday: String = String::from("sunday");
    // let monday: String = String::from("monday");
    // let tuesday: String = String::from("tuesday");
    // let thursday: String = String::from("thursday");
    // let friday: String = String::from("friday");
    // let saturday: String = String::from("saturday");

    match day {
        "sunday" => println!("it's sunday"),
        "monday" => println!("it's monday"),
        "tuesday" => println!("it's tuesday"),
        "thursday" => println!("it's thursday"),
        "friday" => println!("it's friday"),
        "saturday" => println!("it's saturday"),
        _ => println!("invalid day"),
    }

    // ? break and continue statments

    let mut some_number = 19;

    while some_number > 0 {
        
        if some_number == 10 {
            break;
            
        }
        some_number -= 1;
        println!(" the number is {some_number}");
    }
}

fn is_even_odd(number: i32) {
    let res = if number % 2 == 0 { "even" } else { "odd" };
    println!("result is {res}");
}
