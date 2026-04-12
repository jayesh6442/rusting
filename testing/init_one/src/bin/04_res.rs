// fn main() {
//     let res = is_prime(23);
//     println!("{}", res);
// }

// fn is_prime(n: u32) -> bool {
//     if n <= 1 {
//         return false;
//     }

//     for i in 2..n {
//         if n % i == 0 {
//             return false;
//         }
//     }

//     true
// }

fn main() {
    let res = fact(5);
    println!("{}", res);

    let fibo = fibo_of_nth(5);
    println!("{}", fibo);
}

fn fact(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    n * fact(n - 1)
}

fn fibo_of_nth(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    fibo_of_nth(n - 1) + fibo_of_nth(n - 2)
}
