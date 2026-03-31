use std::ptr::addr_of;

fn main() {
    println!("in the borrow chapter");
    let original = String::from("jayesh");

    // ? here the borrow ask to borrow the value and return after use it
    let borrow = &original;
    println!("{}", borrow);
    let _ = borrow.to_uppercase();
    println!("{}", borrow);

    let string_borrow = String::from("jayesh");
    // ? rust always make sure there is no dangling pointer throught the code
    // ? ref should never live out the actual reference
    let string_actual_borrow = &string_borrow;
    println!("{}", string_actual_borrow);

    println!("deref pointer {}", *string_actual_borrow);
    println!("deref pointer {}", &string_actual_borrow);
    println!("address of borrow {:p}", string_actual_borrow);
    let s: String = String::from("test pointer value");
    let ptr = &s;
    println!("address of string [stack] {:p}", ptr);
    println!("address of string [stack] {:p}", &s);
    let raw_ptr = addr_of!(s);
    println!("raw ptr [value] {:p}", raw_ptr);
    println!("address of string [heap] {:p}", s.as_ptr());
}
