use std::collections::HashMap;
use std::collections::HashSet;

fn sequences() {
    let _static_array: [u8; 4] = [0, 1, 2, 3];
    let mut dynamic_array: Vec<u8> = vec![0, 1, 2, 3];
    dynamic_array.push(4);
    println!("{:?}", dynamic_array.pop());
    println!("{:?}", dynamic_array[1]);
    // println!("{:?}", dynamic_array[5]); // panics out of bounds
}

fn maps() {
    let nums: [u8; 4] = [0, 1, 2, 3];
    let next_by_prev: HashMap<&u8,u8> = nums.iter().map(|n| (n, n+1)).collect();
    println!("{} is after {}", next_by_prev[&3], 3);
}

fn sets() {
    let mut cool_numbers = HashSet::from([2, 10, 8]);
    cool_numbers.insert(8); // no effect
    println!("{:?}", &cool_numbers - &HashSet::from([2]));
}

fn main() {
    sequences();
    maps();
    sets();
}
