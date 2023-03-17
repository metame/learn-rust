
// first very failing stab at fizzbuzz without googling anything
// this is a non-standard fizzbuzz where fizzbuzz(3) -> ["1","2","Fizz"], I'll come back to this later
/*
fn fizz_buzz_array(n: i32) -> Vec<String> {
    let a: [String; n] = [];
    for i in 1..=n {
        let r;
        if i % 3 == 0 && i % 5 == 0 {
            r = "FizzBuzz";
        } else if i % 3 == 0 {
            r = "Fizz";
        } else if i % 5 == 0 {
            r = "Buzz";
        } else {
            r = "i";
        }
        a[i] = r;
    }
    return a.to_vec();
}
 */

// ugly as sin and used constants to get around my lack of knowledge of how to use `String`
const fizz: &str = "Fizz";
const buzz: &str = "Buzz";
const fizzbuzz: &str = "FizzBuzz";

fn fizz_buzz (n: i32) -> &'static str {
    if n % 3 == 0 && n % 5 == 0 {
        return fizzbuzz;
    } else if n % 3 == 0 {
        return fizz;
    } else if n % 5 == 0 {
        return buzz;
    } else {
        return "n"; // this is the part that's failing b/c I didn't google how to stringify ints
    }
}

fn main() {
    assert!(fizz_buzz(3) == fizz);
    assert!(fizz_buzz(5) == buzz);
    assert!(fizz_buzz(15) == fizzbuzz);
    println!("You passed!");
}
