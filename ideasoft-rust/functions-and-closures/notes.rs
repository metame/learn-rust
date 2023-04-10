fn main() {
    // as discussed previously, a fn return can either be an expression or return statement
    // these are identical functionally
    fn u32_add_s(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn u32_add_e(a: u32, b: u32) -> u32 {
        a + b
    }

    assert!(u32_add_s(2,3) == u32_add_e(2,3));

    // you can define functions inside other functions

    // you can attach functions to data types whether created with structs or enums
    // an associated function is similar to static functions in java classes, they don't take the type as the first argument and are called similarly as in java
    // methods (just like java class methods) do take the type as the first argument and also called the same as in java

    struct Bop(&'static str);

    impl Bop {
        fn ending() -> &'static str {
            "See you later, Space Cowboy"
        }

        fn character(self: &Self) -> &'static str {
            self.0
        }

        // alternative shorthand syntax for self type
        fn name(&self) -> &'static str {
            self.0
        }
    }

    println!("{}", Bop::ending());
    let spike = Bop("Spike");
    println!("{}", spike.character());
    println!("It's still {}", spike.name());

    // closures that modify mutable references have to also be declared as mutable
    // closures are used to encapsulate environment (akin to the CS def) and often used like lambdas/anonymous fns in other langs
    let a = [1, 2, 3];
    let n: i32 = a.iter().map(|x| x * 2).sum();
    println!("Sum of {:?} after doubling: {}", a, n);
    // e.g. this is used like `x => x*2` would be in js here and doesn't encapsulate anything from the environment

    // this example from the quiz didn't make sense until I tried compiling it. At first glance it seems fine, but a closure actually creates a struct and thus two closures are of two different types
    // fn perform_operation(should_add: bool, amount_to_add: i32, value: i32) -> i32 {
    //         let operation = if should_add {
    //             // A closure that adds `amount_to_add` to `value`.
    //             |value: i32| value + amount_to_add
    //         } else {
    //             // A closure that returns `value` without modification.
    //             |value: i32| value + 0
    //         };

    //         operation(value)
    //     }
}
