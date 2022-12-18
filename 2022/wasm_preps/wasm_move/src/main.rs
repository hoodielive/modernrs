#![allow(unused_variables)]
#![allow(dead_code)]


#[derive(Debug)]
enum PersonID
{
    Passport, 
    IdCard,
    
}

struct Person
{
    name: String,
    last_name: String,
    age: u32,
    id: PersonID,
}

impl Person 
{
    // Associated function.
    fn new() -> Person 
    { 
        Person 
        {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
            id: PersonID::IdCard,
        }
    }

    fn from(name: String, last_name: String, age: u32, id: PersonID) -> Person 
    {
        Person
        {
            name,
            last_name,
            age,
            id,
        }
    }
}

fn main() 
{
    let person = Person::new();
    let person_2 = Person::from(
        String::from("Law"),
        String::from("Osa"),
        32,
        PersonID::IdCard,
    ); 
    
}

