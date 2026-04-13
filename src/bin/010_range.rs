fn main() {
    let some_ints: std::ops::Range<i32> = 1..101;
    for i in some_ints {
        println!("{}", i);
    }
    let arrayone = [1, 2, 4, 5, 7, 8];

    for i in arrayone {
        println!("{}", i);
    }

    let testing = ("one", "one");

    println!("{}",testing.0);
    println!("{}",testing.1);
    
}
