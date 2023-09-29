#![allow(unused_variables)]

fn main()
{
    // String vs &str
   let mut message = String::from("Hello"); // pointer and capacity and length - Heap
   let name = "Osa"; // immutable - pointer but with length only, readonly memory, points to
                     // different area of memory
                    
   message.push_str(" Meji");

   let slice = &message[2..4]; // from index 2 to index 3
   println!("{}", slice); // this is &str (slice) - pointing to heap
}
