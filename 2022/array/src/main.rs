use std::io;
mod ara;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("I need to sing one of these numbers between 0..4");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("The line could NOT be read.");

    let index: usize = index.trim().parse().expect("Index could NOT be found.");

    let element = a[index];

    println!("The value of element is {element}");

    let home = ara::IpAddr::V4(127, 0, 0, 1);
    let loopback = ara::IpAddr::V6(String::from("::1"));

    println!("{:?}, is the local host address.", home);
}
