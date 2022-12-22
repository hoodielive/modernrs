#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum Directions
{
    East, 
    West,
    North,
    South,
}

#[derive(Debug)]
enum TheOdus
{
    Ogbe,
    Oyeku,
    Iwori,
    Idi,
    Irosun,
    Owonrin,
    Obara,
    Okanran,
    Ogunda,
    Osa,
    Ika,
    Oturupon,
    Otura,
    Irete,
    Ose,
    Ofun,
}

#[derive(Debug)]
struct Odus
{
    odu: String,
}

impl Odus
{
    fn new() -> Odus
    {
        Odus {
            odu: "EjiOgbe".to_owned(),
        }
    }
    
    fn display_odu_direction(direction: Directions) -> Directions
    {
       match direction
       {
           Directions::East => println!("Is on the East side and is masculine."),
           Directions::West => println!("Is on the West side and is feminine."),
           Directions::North => println!("Is on the North side and is masculine."),
           Directions::South => println!("Is on the South side and is feminine."),
       }

       direction
    }

    fn display_the_odu(the_odu: TheOdus) -> TheOdus
    {
        TheOdus {
            Ogbe,
            Oyeku,
            Iwori,
            Idi,
            Irosun,
            Owonrin,
            Obara,
            Okanran,
            Ogunda,
            Osa,
            Ika,
            Oturupon,
            Otura,
            Irete,
            Ose,
            Ofun,
        };

        TheOdus
    }
    
}
fn main() 
{
    let primary_odu = Odus::new();
    println!("The primary odu is {:?}", primary_odu);

    
    let primary_odu_direction = Odus::display_odu_direction(Directions::East);
    println!("Its direction is: {:?}", primary_odu_direction);
}
