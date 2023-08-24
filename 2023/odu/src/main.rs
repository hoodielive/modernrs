use std::collections::HashMap;

fn main() {
    let mut odus_map = HashMap::new();

    // Add the odus and their corresponding numbers to the map
    odus_map.insert("Ogbe Meji", 1);
    odus_map.insert("Oyeku Meji", 2);
    odus_map.insert("Iwori Meji", 3);
    odus_map.insert("Idi Meji", 4);

    // Prompt the user to enter an odu
    println!("Enter an odu:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove leading/trailing whitespace from the input
    let input = input.trim();

    // Look up the number corresponding to the entered odu
    if let Some(number) = odus_map.get(input) {
        println!("The number for {} is {}", input, number);
    } else {
        println!("Invalid odu: {}", input);
    }
}
