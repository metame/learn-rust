fn main() {
    //  let a: [u8];   // error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
    //  let a: &[u8];  // error[E0381]: used binding `a` isn't initialized
    let a: &[u8] = &[1,2,3];
    println!("{:?}", a); // prints `[1, 2, 3]`
}
