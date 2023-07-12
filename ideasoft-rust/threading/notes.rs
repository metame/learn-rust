use std::sync::mpsc;
use std::thread;

fn threads(r: std::ops::Range<u8>) {
    let rc = r.clone();
    let handle = thread::spawn(move || {
        for i in r {
            println!("spawned: {i}");
            thread::yield_now();
        }
    });

    for i in rc {
        println!("main: {i}");
        thread::yield_now();
    }

    handle.join().unwrap();
}

fn synchronization() {
    let (tx1, rx) = mpsc::channel();

    let tx2 = tx1.clone();

    thread::spawn(move || {
        tx1.send(1).unwrap();
    });

    thread::spawn(move || {
        tx2.send(2).unwrap();
    });

    println!("{}", rx.recv().unwrap());
    println!("{}", rx.recv().unwrap());
}

fn main() {
    threads(0..5);
    synchronization();
}
