use std::thread;
use std::sync::mpsc::channel;

fn make_vec(start: i32) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..10000 {
        v.push(start + i);
    }
    v
}

// print a vector with Closures
fn print_vec(v: &Vec<i32>, f: &Fn(i32) -> i32) {
    // output the number less than 5, in `v: &Vec<i32>`
    for i in v.into_iter().filter(|x| **x < 5) {
        print!("{} ", f(*i));
    }
    println!();
}

fn main() {
    // channels
    let (tx, rx) = channel();

    // transmit to `channels`
    for i in 0..5 {
        let ttx = tx.clone();
        // let v = make_vec(i);
        thread::spawn(move || {
            ttx.send(make_vec(i)).unwrap();
        });
    }

    // received from `channels`
    for _ in 0..5 {
        let rv = rx.recv().unwrap();
        print_vec(&rv, &|x| x);
    }
}
