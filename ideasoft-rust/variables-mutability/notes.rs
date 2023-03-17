fn main() {
    // destructuring tuple
    let (a, b) = (1, 2);
    println!("a is {} and b is {}", a, b);

    // destructuring struct
    struct Bear {
        kind: &'static str,
        age: u32,
        eats_people: bool,
    }

    let b = Bear {
        kind: "Black",
        age: 10,
        eats_people: false,
    };

    // besides it erring for an incomplete pattern, also leaving this here as I originally used the reserved word `type`
    // but I always would prefer to rename than "escape", which is apparently done with `r#`
    // let Bear {r#type, age} = b; // error[E0027]: pattern does not mention field `eats_people`
    let Bear {kind, age, ..} = b;

    println!("Look ma! It's a {} year old {} bear!", age, kind);

    // warning: field `eats_people` is never read (love this compiler warning)

}
