fn find_matching_elements(array: &[i32], input: i32) -> Vec<i32> {
    // Create an empty result vector
    let mut result = Vec::new();

    // Loop through the array
    for element in array {
        // Check if the current element matches the input
        if *element == input {
            // If it does, add it to the result vector
            result.push(*element);
        }
    }

    // Return the result vector
    result
}

// Example usage:
let names = [1, 2, 3, 4];
let matching_elements = find_matching_elements(&names, 3);
println!("{:?}", matching_elements); // prints [3]
                                     //
                                     
// Prompt user input
use std::io;

fn main() {
    // Prompt the user for input
    println!("Enter your name: ");

    // Read the user's input
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    // Remove the newline character from the end of the input
    name = name.trim().to_string();

    // Print the input back to the user
    println!("Your name is: {}", name);
}
