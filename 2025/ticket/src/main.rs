#![allow(dead_code)]
#[derive(Debug)]
struct Ticket
{
    title: String, 
    description: String, 
    status: String,
}

impl Ticket
{
    fn is_open(&self) -> bool
    {
        self.status == "Open"
    }
    
    fn is_open_test(&self) -> bool
    {
        self.status == "Close"
    }
}

#[derive(Debug)]
struct Configuration
{
    version: u32,
    active: bool,
}

impl Configuration
{
    fn default() -> Configuration
    {
        Configuration { version: 0, active: false }
    }
}

fn main() 
{
    let ticket = Ticket
    {
        title: "Build a ticket system".into(),
        description: "A Kanban board".into(),
        status: "Open".into(),
    };

    let is_open_test = Ticket::is_open_test(&ticket);
    let _x = &ticket.description;
    let is_open = &ticket.is_open();

    println!("None of your business: {}", is_open);
    println!("None of your business 2: {}", is_open_test);

    let default_config = Configuration::default();

    println!("Hello default: {:?}", default_config);
    
    let structure = Configuration
    {
        version: 43,
        active: false,
    };

    println!("The structure is {:?}", structure);

   #[cfg(test)] 
   mod tests
   {
       // [...]
   }
}
