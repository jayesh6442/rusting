fn main() {
    let sum = sum(23, 23);
    println!("{}", sum);
    let res = implicit_return();
    println!("{}", res);
    let jay = String::from("jayesh");
    let upper_case = to_upper_case(&jay);
    println!("{}", upper_case);
    println!("{}", upper_case);
    let return_from_tupal = tupal_return();
    println!(
        "{} {} {}",
        return_from_tupal.0, return_from_tupal.1, return_from_tupal.2
    );
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn implicit_return() -> String {
    String::from("this is fn returned string")
}

fn to_upper_case(edit: &String) -> String {
    edit.to_uppercase()
}

fn tupal_return() -> (String, bool, char) {
    (String::from("jayesh"), false, '2')
}
