#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
enum Access {
    Partial,
    Full,
}

fn dopetuples(x: i32, y: i32, z: i32) -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    let (employee, access) = ("Larry", Access::Full);

    println!("Mr. {:?}'s access is {:?}.", employee, access)
}
