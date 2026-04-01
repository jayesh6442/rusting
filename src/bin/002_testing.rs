use std::ptr::dangling;

fn main() {
    let a = String::from("jayesh");
    // let b = a; =>  // ?invalid
    let b = a.clone();
    println!("{}", a);
    println!("{}", b);

    let mut a_mut = String::from("jayesh");
    a_mut.push_str(" is developer");
    println!("{}", a_mut);
    let ch = '3';
    let second_c = ch; // ? implement copy trait
    println!("{}  {}", ch, second_c);
    let test_var = 23;
    display(test_var);
    println!("{}", test_var);
    let test_string = String::from("test value");
    display_two(test_string);
    // println!("{}",test_string); // ? invalid because the owner ship is trasfered
}

// ? fn chagnes test owner

fn display(a: i32) {
    println!("{}", a);
}

fn display_two(a: String) {
    println!("{}", a);
}
