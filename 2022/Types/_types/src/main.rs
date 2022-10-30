#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod tuples;

fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

fn main() {
    build_vector();

    tuples::tuples();
}
