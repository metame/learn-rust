// a String is owned UTF-8
// a str is borrowed UTF-8

fn mod_str(s: String) -> String {
    let mut mut_str = String::from(s);
    mut_str.push('w');
    mut_str.push_str("orld!");
    mut_str
}

fn main() {
    let h = String::from("Hello, ");
    println!("{h}");
    let hw = mod_str(h);
    println!("{hw}");
    println!("{:?}", "a".as_bytes());
    println!("{:?}", "😊".as_bytes());

}
