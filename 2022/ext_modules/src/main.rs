#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use modad::mathematics::*;
use modad::messaging::*;

fn main() {
    // Part 1: math functions
 
    let result = {
        let two_plus_two = add(2, 2);
        let three = sub(two_plus_two, 1);
        mul(three, three)
    };

    // Ensure we have a correct result.
    assert_eq!(result, 9);
    println!("(2 + 2 - 1) * 3 = {}", result);

    // Part 2: string functions
    let hello = {
        let msg = "hello ";
        let msg = trim(msg);
        capitalize(msg)
    };
    let world = {
        let msg = "world";
        exciting(msg)
    };
    let msg = format!("{}, {}", hello, world);

    // Ensure we have a correct result.
    assert_eq!(&msg, "Hello, world!");
    println!("{}", msg);
}
