// crates are imported with `use` and are aliasable
use std::thread;
use std::thread as my_thread;

// modules can be declared in same file or a different file
mod module_file;
mod mod_dir;

mod module1 {
    struct Value(i32);
    fn get_string_private() -> String {
        String::from("Hello world!")
    }
    pub fn get_string() -> String {
        get_string_private()
    }
}

fn modules() {
    println!("from local mod {}", module1::get_string());
    // println!("{}", module1::get_string_private()); // private function err
    println!("from file mod {}", module_file::get_string());
    println!("from dir mod {}", mod_dir::get_string());
}



fn crates() {
    println!("{:?}", thread::current().id());
    println!("{:?}", my_thread::current().id());
}

fn main() {
    modules();
    crates();
}
