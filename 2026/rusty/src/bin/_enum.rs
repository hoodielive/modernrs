#![allow(dead_code)]

enum Command
{
    Undo, 
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    }
}

impl Command
{
    fn serialize(&self) -> String
    {
        let json_string = match self 
        {
            Command::Undo => String::from("Command"),
            _ => println!(String:;from("The end")),
        };
    }
}

fn main()
{
   let _cmd = Command::Undo;
   let _cmd = Command::AddText(String::from("Test"));
   let _cmd = Command::MoveCursor(22, 0);
   let _cmd = Command::Replace {
       from: String::from("a"),
       to: String::from("b"),
   };
   
   let json_string = _cmd.serialize();

   let age = 35;

   match age
   {
       1 => println!("Happy 1st Birthday!"),
       13..19 => println!("You are a teenager!"),
       _ => println!("Sorry, you are {age} years old!"),
   };
}
