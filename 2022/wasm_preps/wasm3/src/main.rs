#![allow(unused_variables)]

fn main()
{
   let mut message = String::from("Hello");
   let message_2: &mut String = &mut message; // in the background I am being de-referenced

   message_2.push_str(" World");

   println!("{}", message_2);
   println!("{}", message); // you will have to run it second as you can't borrow it immutablely
                            // and then mutablely. Or comment out one or the other.
}
