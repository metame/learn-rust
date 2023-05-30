// just used as an example of a simplistic option type
// #[derive(Debug)]
// enum Option<T> {
//     Some(T),
//     None,
// }

fn first_element<T>(array: &[T]) -> Option<&T> {
    if array.len() > 0 {
        Option::Some(&array[0])
    } else {
        Option::None
    }
}

fn main() {
    let a = [1, 2, 3];
    let first_from_a = first_element(&a);
    println!("{first_from_a:?}");

    let b: [i32; 0] = [];
    let first_from_b = first_element(&b);
    println!("{first_from_b:?}");
}
