#![allow(unused_variables)]

fn main()
{
    let message = String::from("Hello"); // I am a smart pointer
    let message_2: &String = &message; // message_2 is pointing to the ptr while message points to the
                              // heap
                              // message_2 is not owner of the data, it is merely 'borrowing'
                              // ref to message
    
    let message_3: &String = &message;
    
    println!("{}", message);
    println!("{}", message_2);
}
// message and message_2 going out of scope
// message_2 is not dropped because it doesn't own what it references
// message is dropped
