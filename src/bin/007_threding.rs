// use std::{
//     thread::{self, sleep},
//     time::Duration,
// };
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..=10 {
//             println!("hi number  {i} from the spawned thread!");
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     sleep(Duration::from_secs(1));
//     handle.join().unwrap();
//     for i in 1..=5 {
//         println!("hi number  {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }
// }

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();
    let t1 = tx.clone();
    // let v = vec![1, 4, 6];
    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:?}", v);
    // });

    // handle.join().unwrap();
    thread::spawn(move || {
        // let val = String::from("jayesh");
        // tx.send(val).unwrap();
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            t1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("move"),
            String::from("data"),
            String::from("from "),
            String::from("chan"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
    // let rec = rx.recv().unwrap();
    // println!("Got: {}", rec);
}
