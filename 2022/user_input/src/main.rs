#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io;
mod power;

fn get_input() -> io::Result<String>
{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main()
{
   let mut all_input = vec![];
   let mut times_input = 0;

   while times_input < 2
   {
       match get_input()
       {
           Ok(words) => 
           {
                all_input.push(words);
                times_input += 1;
           },
           Err(e) => println!("error: {:?}", e),
       }
   }

   for input in all_input
   {
       println!(
           "Original: {:?}, capitalized: {:?}", 
           input,
           input.to_uppercase()
       );
   }
    
   let mut user_input_for_power = vec![];
   let mut times_input_power = 0;

   while times_input_power < 2
   {
        match power::power_options(String::from("Off"))
        {
            Ok(selection) => 
            {
                user_input_for_power.push(selection);
                times_input_power += 1;
            }
            Err(e) => println!("You have made the following error: {:?}", e),
            
        }
   }

   for input_power in user_input_for_power
   {
       println!("You have selected: {:?}", input_power);
   }
}
