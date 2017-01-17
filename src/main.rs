use std::thread;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

const N: i32 = 5;

fn make_vec(start: i32) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..10000 {
        v.push(start + i);
    }
    v
}

// print a vector with Closures
fn print_vec(v: &Vec<i32>, f: &Fn(i32) -> i32) {
    // output the first 5 number, in `v: &Vec<i32>`
    print!("fear_channels() Vector_{}: ", v[0]);
    for i in 0..N {
        print!("{} ", f(v[i as usize]));
    }
    println!();
}

fn fear_channels() {
    // channels
    let (tx, rx) = channel();

    // transmit to `channels`
    for i in 0..N {
        let ttx = tx.clone();
        // let v = make_vec(i);
        thread::spawn(move || {
            ttx.send(make_vec(i)).unwrap();
        });
    }

    // received from `channels`
    for _ in 0..N {
        let rv = rx.recv().unwrap();
        print_vec(&rv, &|x| x);
    }
}

fn fear_locks() {
    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = channel();
    for _ in 0..N {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
        });
    }

    rx.recv().unwrap();
}

fn main() {
    fear_channels();
    fear_locks();
}
