use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Odu {
    name: String,
    element: String,
    direction: String,
    meaning: String,
}

fn main() {
    let mut odus = HashMap::new();

    odus.insert("ogbe".to_string(), Odu {
        name: "ogbe".to_string(),
        element: "fire".to_string(),
        direction: "east".to_string(),
        meaning: "open road".to_string(),
    });
    // Add the other 15 odus in the same manner

    println!("Enter the name of an odu:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_lowercase();

    if let Some(odu) = odus.get(&input) {
        println!("Name: {}", odu.name);
        println!("Element: {}", odu.element);
        println!("Direction: {}", odu.direction);
        println!("Meaning: {}", odu.meaning);
    } else {
        println!("Invalid odu entered");
    }
}

