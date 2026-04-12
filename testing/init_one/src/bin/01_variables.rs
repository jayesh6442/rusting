fn main() {
    println!(
        "ooo hi there we are in the  rust code base an we are going to be doing some cool stuff"
    );
    let a = 23;
    let b = 23;
    static GOB: i32 = 23;
    println!("{}", a + b);
    println!("{}", GOB);
    let (one, two) = (23, 23);
    println!("{}", one);
    println!("{}", two);

    let Some(x) = Some(String::from("hi there we are in the string")) else {
        return;
    };
    println!("{}", x);

    // ? this upper var are immutable then can not chage

    // ! here are mut ones

    let mut test_strings = "jayesh".to_owned();
    println!("{:?}", test_strings);
    test_strings.push_str("hi there ");
    println!("{}", test_strings);
}
