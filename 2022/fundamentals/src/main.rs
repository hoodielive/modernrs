
fn main() {
    let spaces = "        ";

    // Shadowing.
    let spaces = spaces.len();

    println!("The total amount of spaces is: {spaces}");

    // if you use parse() make sure to include the Result<String,Err>
    // .expect("Response.");

    let guess: u32 = "42".parse().expect("Whhhhhaaaaaa????");

    println!("Guess is: {guess}");

    let overflow_var: u16 = 257;
    println!("{}", overflow_var);
}
