fn main() {
    let s = String::from("hello world");

    let part_onw = &s[0..5];
    let part_two = &s[6..11];
    println!("{}",part_onw);
    println!("{}",part_two);
}
// fn first_word(s: String) -> usize {
//     let bye = s.as_bytes();

//     for (i, &item) in bye.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//         s.len();
//     }
// }
