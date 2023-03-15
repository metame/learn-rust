#![allow(unused)]
fn main() {
    println!("yay referential types!");

    // playing with type inference

    struct Config {
        port: u16,
    }
    let config = Config { port: 8080 }; // inferred Config as type
    // Create a reference.
    let config_reference = &config; // inferred &Config as type
    // In some other part of the program, use the reference.
    println!("Using port {}.", config_reference.port);

    // non copyable types (viz. heap references)
    let hello: &str = "Hello!";
    let val: String = hello.to_string();
    let r1: &String = &val;
    // let val2: String = *r1; // error[E0507]: cannot move out of `*r1` which is behind a shared reference
    let val2: &str = &*hello;
    println!("{} is {} is {}", hello, val, val2);

    let mut n: u8 = 99;
    let mut r: u8 = n;
    r+=1;
    println!("{n}");
}
