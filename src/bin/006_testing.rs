fn main() {
    let mut x = 10;
    let y = &mut x;
    *y = 11;
    println!("{}", x);
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
