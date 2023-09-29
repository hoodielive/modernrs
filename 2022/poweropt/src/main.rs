#![allow(dead_code)]
use std::io;

enum PowerOptions
{
    Off,
    Reboot,
    Shutdown,
    Hibernate,
    Sleep,
}

trait Power
{
    fn new(&self) -> io::Result<String>;
}

impl Power for PowerOptions
{
    fn new(&self) -> io::Result<String>
    {
        let mut buffer = String::new();

        io::stdin()
            .read_line(&mut buffer)?;
            
    }
}

fn main()
{
    
}
