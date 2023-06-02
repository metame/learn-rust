// basic example of associated functions

#[derive(Debug,Clone)]
struct MyType(i32);

impl MyType {
    const DEFAULT: i32 = 0;
    fn new() -> MyType {
        MyType(MyType::DEFAULT)
    }
    fn get(&self) -> i32 {
        self.0
    }
    fn set(&mut self, val: i32) {
        self.0 = val
    }
}

fn associated_functions() {
    let new_mt = MyType(2);
    println!("{new_mt:?} has value {}", new_mt.get());
    let mut mt_clone = new_mt.clone();
    mt_clone.set(3);
    println!("{mt_clone:?} has value {}", mt_clone.get());
    // why bother with a `new` at all... ever
    println!("{:?} has same value as {:?}", MyType::new(), MyType(MyType::DEFAULT));
}


// associated types
struct Index(i32);
trait Joining {
    // Associated types
    type A;
    type B;
    fn join_to_str(&self, _: &Self::A, _: &Self::B) -> String;
}
impl Joining for Index {
    type A = String;
    type B = String;
    fn join_to_str(&self, name: &Self::A, last_name: &Self::B) -> String {
        format!("{}. {} {}", self.0, name, last_name)
    }
}
fn get_joined_str<J: Joining>(joining: &J, name: &J::A, last_name: &J::B) -> String {
    format!("Person: {}", joining.join_to_str(name, last_name))
}
// above fn without associated type, viz. explicitly parameterized by types
fn get_joined_str1<A, B, J:Joining<A=A,B=B>>(joining: &J, name: &A, last_name: &B) -> String {
    format!("Person: {}", joining.join_to_str(name, last_name))
}

fn associated_types () {
    let index = Index(10);
    println!("{}", get_joined_str(&index, &"John".to_string(), &"Connor".to_string()));
    println!("{}", get_joined_str1(&index, &"John".to_string(), &"Connor".to_string()));
}


// associated constants
trait ConstantValue {
    const VALUE: i32;
}
struct MyStruct;

impl ConstantValue for MyStruct {
    const VALUE: i32 = 10;
}

fn associated_constants() {
    println!("{}", MyStruct::VALUE);
}

fn main() {
    associated_functions();
    associated_types();
    associated_constants();
}
