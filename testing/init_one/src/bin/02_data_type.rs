fn main() {
    // println!("{}", 1);
    let one = true;
    let two = true;

    let finalay = one && two;

    println!("{}", finalay);

    let mut year: i32 = 2025;
    println!("{}", year);

    year += 5;
    println!("{:?}", year);
    year -= 5;
    println!("{:?}", year);
    year /= 5;
    println!("{:?}", year);
    year *= 5;
    println!("{:?}", year);

    let age = 23;
    if age != 23 {
        println!("not 23")
    } else {
        println!("age 23")
    }

    let sachme = false;
    if !sachme {
        println!("ye juth he bhai ")
    }

    let darkie: std::ops::Range<i32> = 1..31;

    for i in darkie {
        println!("{}", i);
    }

    let season: [&str; 4] = ["summer", "winter", "rainy", "autumn"];

    println!("{season:#?}");

    dbg!(season);

    // ? this are tupals

    let tupalo: (&str, i32, bool) = ("jayesh", 23, true);

    println!("{:?}", tupalo.0);

    let (name, age, is_good) = tupalo;

    println!("{name} {age} {is_good}");
    let long_one: i64 = 23_23_323_23_23;
    println!("{long_one}");

    let final_int: i32 = 1338;
    let another_one = final_int as i32;
    println!("{}", another_one);
    
}
