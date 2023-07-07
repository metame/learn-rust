use std::{fs, io};
use fs::File;
use io::prelude::*;

fn create() -> io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

fn read() -> io::Result<()> {
    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    Ok(())
}

fn print_read_result(r: io::Result<()>) {
    match r {
        Result::Ok(()) => println!("Read succeeded!"),
        Result::Err { .. }  => println!("Read failed :("),
    }
}

fn print_create_result(r: io::Result<()>) {
    match r {
        Result::Ok(()) => println!("Create succeeded!"),
        Result::Err { .. } => println!("Create failed :("),
    }
}

fn files() {
    print_read_result(read());
    print_create_result(create());
    print_read_result(read());
}

fn read_dirs() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let prefix = match file_type {
            t if t.is_dir() => "D",
            t if t.is_file() => "F",
            t if t.is_symlink() => "L",
            _ => "?",
        };
        println!("{prefix} {}", entry.path().display());
    }

    Ok(())
}


fn dirs() {
    match read_dirs() {
        Result::Ok(()) => println!("Read directories!"),
        Result::Err { .. } => println!("Read directories failed :("),
    }
}

fn main() {
    files();
    dirs();
}
