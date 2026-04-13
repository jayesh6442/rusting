#[derive(Debug)]
enum Testing {
    One(String),
    Two(String),
    Three(String),
}
fn main() {
    println!("test");
    let p1 = Testing::One("one".to_string());
    let p2 = Testing::Two("two".to_string());
    let p3 = Testing::Three("three".to_string());
    match p1 {
        Testing::One(s) => println!("{}", s),
        Testing::Two(s) => println!("{}", s),
        Testing::Three(s) => println!("{}", s),
    }
    println!("{:?}", p2);
    println!("{:?}", p3);
}
