fn main() {
    let a = identity(23);
    println!("{}", a);
    // ? turbofish op

    let b = identity::<String>(String::from("a"));
    println!("{}", b);
    let (a, b) = identity_two(String::from("jayesh"), 23423);
    println!("{} {}", a, b);
}

fn identity<T>(value: T) -> T {
    value
}

fn identity_two<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}
