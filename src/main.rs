use std::thread;
use std::sync::mpsc::channel;

fn make_vec(start: i32) -> Vec<i32> {
    let mut v = Vec::new();
    v.push(start);
    v.push(start+1);
    v
}

// print a vector with Closures
fn print_vec(v: &Vec<i32>, f: &Fn(i32) -> i32) {
    for i in v.iter() {
        print!("{} ", f(*i));
    }
    println!();
}

// channels

fn main() {
    let (tx, rx) = channel();
    for i in 0..5 {
        let ttx = tx.clone();
        let v = make_vec(i);
        thread::spawn(move || {
            ttx.send(v).unwrap();
        });
    }

    for _ in 0..5 {
        let rv = rx.recv().unwrap();
        print_vec(&rv, &|x| x);
    }
}
