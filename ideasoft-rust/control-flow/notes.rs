fn main() {
    // interesting that the compiler and clippy warned on a struct value I never used but don't warn on whole conditional blocks that will never run
    if true {
        println!("always this one!");
    } else if false {
        println!("never this one!");
    } else {
        println!("like still not going to print!");
    }

    // loops
    loop {
        println!("I can't stop!");
        break;
    }

    {
        let mut i = 3;
        loop {
            if i == 0 {
                break;
            }
            println!("{i}...");
            i -= 1;
        }
        println!("Launch!");
    }

    {
        let mut i = 3;
        while i != 0 {
            println!("{i}...");
            i -= 1;
        }
        println!("Launch!");
    }

    {
        // 1..=3 is inclusive range, 1..3 is exclusive range
        for i in (1..=3).rev() {
            println!("{i}...");
        }
        println!("Launch!");

        // can also use `continue` to skip, just like in other langs
        for i in (1..=3).rev() {
            if i % 2 == 0 {
                continue;
            }
            println!("{i}...");
        }
        println!("Launch!");
    }

    // ^^ because of the nice iterator interfaces the for loop is definitely the nicest of the 3 here

}
