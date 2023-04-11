use std::fmt::Display;

#[derive(Debug)]
struct MyStruct<A, B> {
    a: A,
    b: B,
}

impl<A: Display, B: Display> MyStruct<A, B> {
    fn print(&self) {
        println!("a: {}, b: {}", self.a, self.b)
    }
}

fn main() {
    let mystruct = MyStruct::<i32, String> {a: 1, b: "a".to_string()};
    mystruct.print();

    println!("Debugging {:?}", mystruct);
}
