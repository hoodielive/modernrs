#![allow(unused_variables)]
#![allow(dead_code)]

enum Paths {
    Buddhism(String, String),
    Christianity(String, String),
    Ifa(String, String),
    Taoism(String, String),
    Islam(String, String),
}

fn main() {
    let path = vec![
        Paths::Buddhism(String::from("Japan"), String::from("Dogen")),
        Paths::Christianity(String::from("Eastern"), String::from("Gnosticism")),
        Paths::Ifa(String::from("West Africa"), String::from("Isese")),
        Paths::Taoism(String::from("Japan"), String::from("Shintoism")),
        Paths::Islam(String::from("Kemet"), String::from("Neteru")),
    ];

    for your_path in path
    {
        let solly = path[0];

        match your_path {
           Paths::Buddhism(location, practice) => println!("You are practicing {:?} from {:?}.", practice, location),
           Paths::Christianity(location, practice) => println!("You are practicing {:?} from {:?}.", practice, location),
           Paths::Ifa(location, practice) => println!("You are practicing {:?} from {:?}.", practice, location),
           Paths::Taoism(location, practice) => println!("You are practicing {:?} from {:?}.", practice, location),
           Paths::Islam(location, practice) => println!("You are practicing {:?} from {:?}.", practice, location),
           _ => println!("No clue what you are practicing."),
        }
    };
}
