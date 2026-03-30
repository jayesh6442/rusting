/*
    * we are in this part where we are going to learn about the data type in rust
    ? we have this data type in rust coding
*   i8
*   i16
*   i32
*   i64
*   i128


* for teh float we have this capacity in rust
*   f32
*   f64

   * then comes the bool type
*   bool

? fro the chat type we have chat datatype in rust
* char

? this are some more
* tupal and array and vector, slices

? string type
 * we have two type of string in rust code
 * &str => this is binary imbeded in to code at compile time
 * String => this kind of string are stored in heap of the memory stack

? then comes the struct
* this is one data that holds other data like container
? then comes enum
? then generics


 */
fn main() {
    println!("hi there we are in the rust coding part");

    let a: i8 = 34;
    let b: i16 = 23;
    let c: i32 = 23;
    let d: i128 = 234234324234;
    println!("{} and {} and {} and {}  ", a, b, c, d);

    let e: isize = -23423414141;
    let f: usize = 312112313123;

    println!(" this is isize {} and this is usize  {}", e, f);

    // ? this jkare unsigned that means this can't be nagative
    let g: u8 = 234;
    let h: u16 = 234;
    let i: u32 = 234;
    let j: u128 = 234;

    println!(" {}  {} {} {}", g, h, i, j);

    // ? flosts

    // let k: f16 = 23.2; => i don't know why rust compiler are not allowed to do this
    let l: f32 = 34.23423423424;
    let m: f64 = 234.2342342342342;
    // let n: f128 = 123.2413413;

    println!("{} {}", l, m);

    let o: bool = false;
    let p: bool = true;
    println!("{} {}", o, p);

    let character: char = 'a';
    let string_example = "he he he ";

    println!("{}  {}", character, string_example);
    let another_string_example = String::from("he he he ");
    println!("{}", another_string_example);

    let test_tuple = (32, false, "jayesh");
    #[allow(unused_variables)]
    let fixed_type_tuple: (bool, bool, i64) = (false, true, 23);

    println!("{}", test_tuple.0);
    println!("{}", test_tuple.1);
    println!("{}", test_tuple.2);
    // ? arrays

    let arr: [bool; 5] = [false, false, true, true, false];
    let one = String::from("one");
    let two = String::from("two");
    let three = String::from("three");
    let string_arr: [String; 3] = [one, two, three];

    for i in arr {
        println!("{}", i);
    }

    for i in string_arr {
        println!("{}", i);
    }
}
