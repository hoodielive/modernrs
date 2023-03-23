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

enum Orientations
{
    Masculine,
    Feminine,
}

#[derive(Debug)]
struct TheOdus
{
    ogbe: String,
    oyeku: String,
    iwori: String,
    idi: String,
    irosun: String,
    owonrin: String,
    obara: String,
    okanran: String,
    ogunda: String,
    osa: String,
    ika: String,
    oturupon: String,
    otura: String,
    irete: String,
    ose: String,
    ofun: String,
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
            ogbe: "ogbe".to_string(),
            oyeku: "oyeku".to_string(),
            iwori: "iwori".to_string(),
            idi: "idi".to_string(),
            irosun: "irosun".to_string(),
            owonrin: "owonrin".to_string(),
            obara: "obara".to_string(),
            okanran: "okanran".to_string(),
            ogunda: "ogunda".to_string(),
            osa: "osa".to_string(),
            ika: "ika".to_string(),
            oturupon: "oturupon".to_string(),
            otura: "otura".to_string(),
            irete: "irete".to_string(),
            ose: "ose".to_string(),
            ofun: "ofun".to_string(),
        }
    }
    
}

fn main() 
{
    let primary_odu = Odus::new();
    println!("The primary odu is {:?}", primary_odu);

    
    let primary_odu_direction = Odus::display_odu_direction(Directions::East);
    println!("Its direction is: {:?}", primary_odu_direction);
}
