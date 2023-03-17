fn main() {
    enum Snack {
        Pretzels { filled: bool },
        Chips { ounces: f64 },
    }

    let snack = Snack::Pretzels { filled: true };
    // let Snack::Pretzels { filled } = snack; // error[E0005]: refutable pattern in local binding
    //     ^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `Snack::Chips { .. }` not covered

    if let Snack::Pretzels { .. } = snack {
        println!("I'm having pretzels!");
    }

    // interestingly, the course showed how specifying `filled` above and not using it would result in compiler warning
    // but they didn't actually demonstrate that we were creating a scoped variable to the inner block... missed opportunity there
    if let Snack::Pretzels { filled } = snack {
        println!("I'm having{} pretzels!", if filled {" filled"} else {""});
    }

    // println!("filled is {}", filled); // error[E0425]: cannot find value `filled` in this scope

    if let Snack::Pretzels { filled: true } = snack {
        println!("I'm having filled pretzels");
    }


    // another example of unneccessarily confusing people
    // original example
    for n in 0..=5 {
        match n {
            1 => println!("It was one!"),
            2 => println!("It was two!"),
            // or-pattern
            3 | 4 => println!("It was a bit more than two!"),
            // match guard
            high if high >= 5 => println!("It was a high number!"),
            // a pattern consisting only of the name `other`
            other => println!("It was {other}!"),
        }
    }

    // clearer example (e.g. the match guard and actually showing the default)
    for n in 0..=7 {
        match n {
            1 => println!("It was one!"),
            2 => println!("It was two!"),
            // or-pattern
            3 | 4 => println!("It was a bit more than two!"),
            // match guard
            high if high >= 5 && high < 7 => println!("{high} is a high number!"),
            // a pattern consisting only of the name `other`
            other => println!("It was {other}!"),
        }
    }


    match snack {
        Snack::Pretzels { filled } if filled => {
            println!("I had filled pretzels!");
        }
        Snack::Pretzels { .. } => {
            println!("I had pretzels!")
        }
        Snack::Chips { .. } => todo!(),
    }

    let mut microwaved_snack = Snack::Pretzels { filled: false };

    while let Snack::Pretzels { .. } = microwaved_snack {
        println!("Having pretzels!");
        microwaved_snack = Snack::Chips { ounces: 12.0 };
    }
    println!("Finally eating chips!");

    // remarkably, one of the quiz questions seems to be completely incorrect and doesn't even compile. I pointed this out to the course creators.

    enum Key { Up, Down, Left, Right }

    // the original match statement which doesn't compile
    // error[E0170]: pattern binding `Up` is named the same as one of the variants of the type `main::Key`
    // help: to match on the variant, qualify the path: `main::Key::Up`
    // match Key::Left {
    //     Up => println!("Jumping"),
    //     Down => println!("Ducking!"),
    //     Left => println!("Sliding Left!"),
    //     Right => println!("Sliding Right!"),
    // }

    // so I changed it to:
    match Key::Left {
        Key::Up => println!("Jumping"),
        Key::Down => println!("Ducking!"),
        Key::Left => println!("Sliding Left!"),
        Key::Right => println!("Sliding Right!"),
    }
    // which naturally prints "Sliding Left!"
    // the correct answer according to the course was "Jumping"
    // interested to hear what the course creator has to say
}
