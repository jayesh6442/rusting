fn main() {
    let space = "   ";
    let lenth = space.len();
    println!("{}", lenth);

    let months = ["jan", "feb", "march", "april", "may"];
    for i in months.into_iter() {
        println!("{}", i);
    }

     test_value();
    for i in (1..100).rev(){
        print!(" {}",i);
    }
    }

#[allow(unused_variables)]
fn test_value() {
    let y = {
        let mut x = 3;
        x += 4;
        x
    };
    println!("{:?}", y);
}
