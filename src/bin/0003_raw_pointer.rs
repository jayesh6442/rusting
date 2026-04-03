use std::{env::args, fs::File, intrinsics::copy, io::BufReader, time::Instant};

use flate2::{Compression, bufread::GzEncoder};
fn main() {
    if args().len() != 3 {
        println!("Usage: {} <input> <output>", args().nth(0).unwrap());
        return;
    }
    println!("wea are in the rust code base");

    // let sushi = String::from("sushi");

    let my_box = Box::new(233);
    println!("{}", *my_box);
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let mut output = File::create(args().nth(1).unwrap()).unwrap();
    // let encode = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
}
