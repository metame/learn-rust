use std::fs::File;
use std::panic;

fn main() {
    // panic!("Oh no!");

    match panic::catch_unwind(|| {
        File::open("some-non-existing-file")
            .expect("Unable to open \"some-non-existing-file\"!");
    }) {
        Ok(_) => println!("No panics occurred."),
        Err(_) => println!("The code panicked."),
    }

    println!("The panic won't stop me!");

    // double panic
    struct Boom;

    // drop implementations are called when panic
    impl Drop for Boom {
        fn drop(&mut self) {
            panic!("Boom!");
        }
    }

    let _boom = Boom;
    panic!("Stop!");

    // it's impossible to recover from a double panic
}
