use std::fmt;
use std::path::{Path, PathBuf};
use std::str;

// types of traits

// conversion:

// `From` and `Into` specify infallible value-to-value conversions

pub trait From<T> {
    fn from(value: T) -> Self;
}

pub trait Into<T> {
    fn into(self) -> T;
}

// if you implement From<A> for B then Into<B> is automatically implemented for A
// `From` and `Into` are useful when converting from specific to more generic type
// You should prefer to implement From as Into get automatically derived
// You should prefer to implement Into in generic parameter bounds in case only Into was implemented by the parameter Type

// TryFrom and TryInto naturally are abstractions for fallible val-to-val conversions
// Implementing Display will automatically implement ToString
// std lib recommends not implementing ToString manually
// FromStr parses value from a string

enum Color {
    Blue,
    Yellow,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Color::Blue => "blue",
            Color::Yellow => "yellow",
        })
    }
}

impl str::FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Color::Blue),
            "yellow" => Ok(Color::Yellow),
            _ => Err(()),
        }
    }
}

fn string_converters() {
    println!("{}", Color::Yellow);
    let color = "blue".parse::<Color>().unwrap();
    println!("{}", color);
}

// AsRef & AsMut behave as you'd expect
fn print_path<P: AsRef<Path>>(path: P) {
    println!("{}", path.as_ref().display());
}

fn as_ref() {
    let a: &'static str = "static_str";
    print_path(a);

    let b: String = String::from("owned_string");
    print_path(b);

    let c: PathBuf = PathBuf::from("owned path");
    print_path(c);
}

fn ownership_converters() {
    as_ref();
    // also cover BorrowRef & BorrowMut which are similar but include the Eq, Ord, & Hash guarantees of Borrow
}

// Deref and Deref mut are for implicitly unwrapping values from a wrapper
struct DerefExample<T> {
    value: T
}

impl<T> std::ops::Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn deref_example() {
    let x = DerefExample { value: 'a' };
    println!("a is {}", *x);
}

// the traits Fn, FnOnce, and FnMut are automatically implemented serving mostly as trait bounds for higher order fns

fn twice<F: Fn() -> &'static str>(f: F) {
    println!("{} {}", f(), f());
}

fn closures() {
    twice(|| "One");
}

// Comparison traits include Eq, PartialEq, Ord, PartialOrd, and Hash found in std::cmp

// Iteration traits include Iterator, IntoIterator, FusedIterator & DoubleEndedIterator
#[derive(Debug)]
struct CountDown(usize);

impl Iterator for CountDown {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            self.0 -= 1;
            Some(self.0)
        }
    }
}

fn iterator_trait() {
    let mut iter = CountDown(2);
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    println!("yay {iter:?} iterators are {:?}", iter.next());
}

#[derive(Debug)]
struct DoubleRange(usize);

impl IntoIterator for DoubleRange {
    type Item = usize;
    type IntoIter = std::ops::Range<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0..self.0 * 2
    }
}

fn into_iterator() {
    let mut iter = DoubleRange(3).into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    println!("yay {iter:?} iterators are {:?}", iter.next());
}

fn iterators() {
    iterator_trait();
    into_iterator();
}

// IO traits include Read and Write which abstract sync read/write of bytes from/to mem, files, network, etc.
// The traits need to be imported b/c they're not part of Prelude

// when implementing your Error types you should implement std::error::Error
// community crates like thiserr and eyre reduce burden of custom error types
#[derive(Debug)]
enum Error {
    A,
    B,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::A => "a happened",
            Self::B => "b happened",
        })
    }
}

// The `Error` trait requires Debug & Display but trait methods are optional
impl std::error::Error for Error {}

type MyResult = Result<(), Box<dyn std::error::Error>>;

fn make_error_b() -> MyResult {
    Err(Error::B)?;
    Ok(())
}

fn make_error_a() -> MyResult {
    Err(Error::A)?;
    Ok(())
}

fn print_result(e: MyResult) {
    match e {
        Err { .. } => println!("There was an error!"),
        Ok(()) => println!("It ran ok!"),
    }
}

fn errors() {
    print_result(make_error_a());
    print_result(make_error_b());
    print_result(Ok(()));
}

fn main() {
    string_converters();
    ownership_converters();
    deref_example();
    closures();
    iterators();
    errors();
}
