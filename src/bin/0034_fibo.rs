fn main() {


    let res = fibo_nth(10);
    println!("{}", res);
}

fn fibo_nth(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibo_nth(n - 1) + fibo_nth(n - 2);
    }
}
