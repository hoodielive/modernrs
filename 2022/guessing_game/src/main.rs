use rand::Rng;
use std::cmp::Ordering;
use std::io;

<<<<<<< HEAD
fn main() {
    println!("Please guess the number.");

    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    println!("Please enter your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guess: {}", guess);

    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
=======
fn main() {}
>>>>>>> 8cf221c4472f8868c9d13e389aa960b44135d7c2
