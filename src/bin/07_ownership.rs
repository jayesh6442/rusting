fn main() {
    println!("this is ownership chapter");

    {
        let a = "jayesh";
        println!("{}", a);
        // ? this will be cleared at end of this block of code
    }
    // ?no allow to use a here

    let a = "jayesh";
    println!("{a}");
    // ? at the end of this code the a will go out of scope in this fn

    let time = 2025;
    // ? this will be copy the pointer value of the time
    let another_time = time;
    println!("{}", time);
    println!("{}", another_time);

    println!("{}", another_time);

    let string_type_one = "jayesh";
    // ? this is also copy of the pointer
    let another_string_type_two = string_type_one;
    println!("{}", string_type_one);
    println!("{}", another_string_type_two);

    let string_type_one_another = String::from("new one");
    // let strint_ref_new = string_type_one_another;
    // ? this create clone of original ref string

    // let strint_ref_new = string_type_one_another.clone();
    // ? this is also valid
    let strint_ref_new = string_type_one_another.to_owned();
    println!("{}", string_type_one_another); // ? this gives error because the original value is cleared and new one ref created
    println!("{}", strint_ref_new);
    let mut string_that_can_be_updated = String::from("jayesh");

    string_that_can_be_updated.push_str(" update");
    println!("{}", string_that_can_be_updated);

    // ? heap allocated data are don't follow the copy trait so previous value cleared and new one is created
    // ? move the owner ship

    let one_onner = String::from("jayesh");
    let another_owner = one_onner; // ? here the owner ship of the one_onner trasfered to the another_owner and first goes out of scope
    println!("{another_owner}");
    drop(another_owner);
    let pass_on = String::from("one of the vlaue");
    // take_owner(pass_on);
    // println!("{pass_on}");
    remain_original_owner(&pass_on);
    println!("{pass_on}");
    // take_owner(pass_on);
    let test_mut = String::from("update likey to happen");
    unmutable_params(test_mut);
}

fn take_owner(value: String) {
    // ? this thing take the owner ship of passed value and clean up at the end of code and will
    println!("{value}");
}
fn remain_original_owner(value: &String) {
    println!("{value}");
}

fn unmutable_params(mut meal: String) {
    meal.push_str("hi there this is new content");
    println!("updated value of the man {}", meal);
}
