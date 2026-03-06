#![allow(dead_code)]
struct NameLength
{
   name: String,
   length: usize,
}

impl NameLength
{
    fn new(name: &str) -> Self
    {
       Self
       { 
           length: name.len(),
           name: name.to_string(),
       }
    }

    fn print(&self)
    {
        println!("The name '{}' is '{}' characters long", self.name, self.length);
    }
}

fn main()
{
   // We don't need to care about the internal structure
   // of Namelength; Instead we can just call its constructor

    let name_length = NameLength::new("John");

    println!("{}", name_length);
}
