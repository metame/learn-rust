fn main() {
    // rust's difference is a compile-time model of ownership
    // when a value is no longer needed, Rust runs a destructor on the value

    // rust compiler implicitly handles copying values that have implemented Copy trait, e.g.
    {
        let a = 10;
        let _b = a;
        println!("a = {a}");
    }
    // the above example will not work if a is defined as a String
    // for types that don't implement the Copy trait, you can clone
    {
        let a = String::from("a");
        let _b = a.clone();
        println!("a = {a}");
    }
}
