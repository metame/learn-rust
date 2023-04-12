fn main() {
    // declarative macros, very similar to clojure macros afaict,
    // the ! at the end of println! is an indication it's a declarative macro
    // procedural macros have 3 types
    // 1. custom derive macros
    // example is Default, Debug, Clone, PartialEq, these are used for deriving traits
    // 2. attribute-like macros
    // can be applied to any item, transforming the written code into any other code
    // 3. function-like macros
    // can generate ny code from its input code, invoked just like declarative macros with !
}
