struct Value(i32);

fn get_string_private() -> String {
    String::from("Hello world!")
}

pub fn get_string() -> String {
    get_string_private()
}
