fn main() {
    let mul_by = |x: i32| x * x;

    println!("{}", mul_by(4));
    let a = mul_by(6);
    println!("{a}");
    let b = || -> i32 {
        return 4;
    };

    println!("{}", b());
    let str_closure = || "this is returned string by closure";
    println!("{}", str_closure());

    // let fact = |i32| (a + 1);
    // println!("{}", fact());

    let mut nums = vec![12, 34, 1, 523, 13, 43];
    let mut add_nums = || nums.push(23);
    add_nums();
    println!("{:?}", nums);
}
