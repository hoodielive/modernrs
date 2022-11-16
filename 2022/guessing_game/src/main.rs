use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main()
{
    println!("Guess the number!");
    println!("Please enter your number input.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");

    println!("I'd like for you to type some other items into this string.");
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read new shit.");

    println!("In total you entered: {guess}");

}
