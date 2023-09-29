#![allow(unused_variables)]

mod extended;

fn main()
{
    let message = String::from("Hello"); // comes into scope - heap
    print_message(message); // message is moved into print_message function
                            // message is no longer valid

    println!("{}", message); // won't work because message is no longer valid
                             //
    let a_message = String::from("Howdy");
    extended::extend_message(a_message);
    println!("{} won't work because it was moved into extend_message", a_message);
} // message is going out of scope, but nothing more will happen because it was moved into
  // print_message


fn print_message(a: String) // a comes into scope
{
    println!("{}", a);
    let c = a; // c is coming into scope and a is moved into c 
               // a is no longer valid
} // a is going out of scope, but nothing will happen because it was moved
  // c is going out of scope and 'drop' is called which clears underlying memory from the  heap
