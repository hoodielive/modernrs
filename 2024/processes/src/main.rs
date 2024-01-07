// Load Module.
mod mytypes;

// Introduce the Color Type into scope.
use mytypes::Color;

fn main() 
{
    demo_simple_enums()
    
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
}
