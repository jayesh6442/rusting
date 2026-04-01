use std::sync::Mutex;

fn main() {
    println!("testing");
    let ok: Result<i32, &str> = Result::Ok(4);

    let er: Result<i32, &str> = Result::Err("some error");
    println!("{:?}", ok.unwrap());
    println!("{:?}", er.unwrap());
}
