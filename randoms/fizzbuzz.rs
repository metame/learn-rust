
// first very failing stab at fizzbuzz without googling anything
// this is a non-standard fizzbuzz where fizzbuzz(3) -> ["1","2","Fizz"], I'll come back to this later

fn fizz_buzz_array(n: i32) -> Vec<String> {
    let a: &mut [String] = &mut [];
    for i in 1..=n {
        let r;
        if i % 3 == 0 && i % 5 == 0 {
            r = String::from("FizzBuzz");
        } else if i % 3 == 0 {
            r = String::from("Fizz");
        } else if i % 5 == 0 {
            r = String::from("Buzz");
        } else {
            r = n.to_string();
        }
        a[(i - 1) as usize] = r;
    }
    return a.to_vec();
}


// ugly as sin and used constants to get around my lack of knowledge of how to use `String`
// in an actual interview exercise, I'd probabl ditch the constants and use `String::from("fizz")` explicitly
const FIZZ: &str = "Fizz";
const BUZZ: &str = "Buzz";
const FIZZBUZZ: &str = "FizzBuzz";

fn fizz_buzz (n: i32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        String::from(FIZZBUZZ)
    } else if n % 3 == 0 {
        String::from(FIZZ)
    } else if n % 5 == 0 {
        String::from(BUZZ)
    } else {
        n.to_string()
    }
}

fn main() {
    assert!(fizz_buzz(1) == "1");
    assert!(fizz_buzz(3) == FIZZ);
    assert!(fizz_buzz(5) == BUZZ);
    assert!(fizz_buzz(15) == FIZZBUZZ);
    println!("You passed level 1!");

    println!("{:?}", fizz_buzz_array(5));
    assert!(fizz_buzz_array(1) == ["1"]);
    println!("You passed level 2!");

}
