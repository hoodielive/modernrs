// Crate-scope attribute.
#![allow(dead_code)]

// Load Module.
mod mytypes;

// Introduce the Color Type into scope.
use mytypes::{Color, HouseLocation};

fn main() 
{
    demo_simple_enums();
}

fn demo_simple_enums()
{
    println!("Enums...");
    let c: Color = Color::Red;

    match c {
        Color::Red   => println!("Coch."),
        Color::Green => println!("Gwyrdd."),
        Color::Blue  => println!("Glas."),
    }

    let h1: HouseLocation = HouseLocation::Number(4);
    match h1 {
        HouseLocation::Number(n) => println!("The Number is {}", n),
        HouseLocation::Name(s)   => println!("The Name is {}", s),
        HouseLocation::Unknown   => println!("Is Unknown"),
    }

    // std = crate of standard library. 
    // mem = a module (a namespace that defines related things)
    // size_of = function in the mem module. 
    let size = std::mem::size_of::<HouseLocation>();
    println!("The size_of size is {}", size);
}


