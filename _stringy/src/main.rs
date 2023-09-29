#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;
mod closures;
mod functions;
mod hof;
mod ownership;

fn strings() {
    let s: &'static str = "howdy!";

    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("First lett is {}.", first_char);
    }

    // heap/string
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);
    // let u:&str = &letters;
    let z = letters + "abc";
}

fn main() {
    strings();
    functions::functions();
    closures::closures();
}
