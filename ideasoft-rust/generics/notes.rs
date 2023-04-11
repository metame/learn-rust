use std::cmp::PartialEq;
use std::ops::Add;

fn main() {
    // generics look nearly identical to java generics
    struct Sequence3<T> {
        first: T,
        second: T,
        third: T,
    }

    impl<T> Sequence3<T> {
        pub fn new(first: T, second: T, third: T) -> Self {
            Self { first, second, third }
        }
    }

    impl<T: PartialEq> Sequence3<T> {
        // my impl of all_same
        pub fn my_all_same(&self) -> bool {
            self.first == self.second && self.first == self.third
        }
    }

    // alternative impl bounded type to trait syntax
    impl<T> Sequence3<T> where T: Copy + Add<Output = T> {
        pub fn sum(&self) -> T {
            self.first + self.second + self.third
        }
    }

    let intseq = Sequence3::<i32>::new(1, 2 ,3);
    assert!(!intseq.my_all_same());
    let strseq = Sequence3::<String>::new("a".to_string(), "a".to_string(), "a".to_string());
    assert!(strseq.my_all_same());

    assert!(intseq.sum() == 6);

    // with compiler's help, I implemented their exact definition
}
