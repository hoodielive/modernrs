use std::io;

#[derive(Debug)]
pub enum Power
{
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

pub fn power_options_01() -> io::Result<String>
{
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)?;
        
    Ok(buffer.trim().to_owned())
}

pub fn power_options(buffer: String) -> io::Result<String>
{
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)?;
        
   #[allow(non_snake_case)]
    match &buffer
    {
       Off => println!("You chose to turn this thing off."),
       Sleep => println!("You chose to put this thing to sleep."),
       Reboot => println!("You chose to reboot this thing."),
       Shutdown => println!("You chose to shutdown this thing."),
       Hibernate => println!("You chose to hibernate this thing."),
    }

   Ok(buffer.trim().to_owned())
}
