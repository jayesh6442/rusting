enum Status {
    Pending,
    Online,
    Offline,
    Complited,
    Processing,
    Finished,
}

enum Data {
    A(i32),
    B(i64),
}

enum ApiResonse {
    Success { data: String },
    Error { code: u32, message: String },
}
fn main() {
    println!("hi there this is enum file");
    #[allow(unused_variables)]
    let api_response: ApiResonse = ApiResonse::Success {
        data: String::from("here is your data"),
    };
    let api_response: ApiResonse = ApiResonse::Error {
        code: 404,
        message: String::from("not found htis "),
    };

    match api_response {
        ApiResonse::Success { data } => println!("{}", data),
        ApiResonse::Error { code, message } => println!("{} {}", code, message),
        _ => print!("nothing matched for you"),
    }
}
