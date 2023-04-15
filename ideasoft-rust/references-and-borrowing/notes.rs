// interesting part to note here is that you define that this fn only takes (immutable) references,
// instead of expecting the caller to pass you a reference or not depending on their implementation
fn length_of_string(value: &String) -> usize {
    value.len()
}

fn main() {
    let s1 = String::from("Hey there!");
    let len = length_of_string(&s1);
    println!("The length is {len}.");

    let s2 = String::from("Hey you!");
    let len2 = length_of_string(&s2);
    println!("The length of {s2:?} is {len2}");

    let mut s3 = String::from("Hello");
    let w1 = &mut s3;
    println!("Yay, mutable reference of s3 is {w1:?}");
    // will not compile b/c can't have borrow `s3` as mutable more than once at a time (while both are still in scope)
    // let w2 = &mut s3;
    // println!("{w1} and {w2}");
    // still won't compile b/c you can't have coexisting mutable and immutable references (while both are still in scope, see below)
    // let w2 = &s3;
    // println!("{w2} and {w1}");


    // won't compile b/c dangling reference
    // fn dangle() -> &String {
    //     &String::from("hello")
    // }

    fn no_dangle() -> String {
        String::from("hello")
    }

    let s4 = no_dangle();
    println!("{s4} without no dangle");

    // non-lexical lifetimes example aka "where mutable & immutable references look like they coexist", much better title
    let mut s5 = no_dangle();
    let r1 = &s5;
    let r2 = &s5;
    println!("{} and {}", r1, r2);
    // cope of r1/r2 ends here since there's no more usage
    let r3 = &mut s5;
    println!("{r3}");

}
