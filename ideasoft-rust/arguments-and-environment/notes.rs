fn print_args() {
    for argument in std::env::args() {
        println!("{argument}");
    }
}

fn print_env_vars() {
    for (key, value) in std::env::vars() {
        println!("{key}={value}")
    }
}

fn print_cwd() {
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
}

fn main() {
    print_args();
    print_env_vars();
    print_cwd();
}
