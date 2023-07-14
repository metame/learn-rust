mod string_mod;

struct Value(i32);

pub fn get_string() -> String {
    string_mod::get_string()
}
