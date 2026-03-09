use std::path::Path;

#[derive(Debug)]
struct Magician
{
    name: String,
    power: String,
}

impl Magician
{
    fn new(name: String, power: String) -> Magician
    {
        Magician
        {
            name,
            power,
        }
    }
}


fn main()
{
    let magus = Magician::new(String::from("Aleister Crowley"), String::from("Grammar"));
    println!("{:?}", magus);
    
}
