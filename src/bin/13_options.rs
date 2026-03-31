fn main() {
    println!("option and result enum");
    let a = Option::Some(4);
    // let b: Option<bool> = Option::None;
    println!("{:?}", a.unwrap());
    // println!("{:?}", b.expect("error encoutered"));

    let nothing: Option<i32> = None;
    println!("{}", nothing.unwrap_or(0));
    let arr: [i32; 3] = [12, 42, 12];

    let index_one = arr.get(2);
    println!("{:?}", index_one.unwrap());
}
