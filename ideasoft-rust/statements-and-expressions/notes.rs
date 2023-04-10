fn main() {
    // statements are terminated by a semicolon
    // expressions are, or produce, values

    // `if` in rust is like ternary in other langs which is to say it can be used as an expression inside a statement

    // it's interesting to say that statements are useful for producing side effects, even though `let` is a statement
    // I wonder if other rust resources parrot this b/c it doesn't make sense to me
    // so far, it seems clear that you can only produce a side-effect with a statement so in that way it's true
    // but it's not like ALL statements produce side effects

    // a scope/block returns the last expression or result of statement, which allows you to do ugly things like this
    let a = if true {
        println!("`a` is still unassigned");
        3
    } else {
        2
    };
    println!("`a` is now {}", a);
}
