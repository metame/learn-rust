// smart pointers allow multiple owners
// smart pointers are usually implemented with structs, implementing `Deref` and `Drop` traits
// `Deref` allows smart pointer instance to behave like a reference
// `Drop` is called when smart pointer instance goes out of scope

fn heap_int(i: i32) -> Box<i32> {
    Box::new(i)
}

// Box is used to store unsized types, without box this would not compile as unsized types can't be stored on the stack (and slices are unsized)
fn create_data(small: bool) -> Box<[u8]> {
    if small {
        Box::new([1, 2, 3])
    } else {
        Box::new([1, 2, 3, 4, 5])
    }
}

// Rc<T> and Arc<T> are reference shared pointers for multithreaded scenarios and thus immutable by default


// most common smart pointers in std lib are `Box<T>` `Rc<T>` `Ref<T>` `RefMut<T>`
fn main() {
    let b = heap_int(10);
    println!("{b}");
    let r = create_data(true);
    println!("{r:?}");

}
