#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
enum PersonID
{
    Passport(u32, u32, u32), 
    IdCard(u32),
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
            id: PersonID::IdCard(0),
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

    fn display_info(&self)
    {
        println!("{}{}{}{:?}", self.name, self.last_name, self.age, self.id)
    }
}

fn main() 
{
    let person = Person::new();
    let person_2 = Person::from(
        String::from("Law"),
        String::from("Osa"),
        32,
        PersonID::IdCard(44),
    ); 

    check_person_id(person.id);
    check_person_id(person_2.id);
    
}

fn check_person_id(id: PersonID)
{
    match id
    {
        PersonID::IdCard(x) =>
        {
            println!("ID card: first value - {}", x)
        },
        PersonID::Passport(x, y, z) => {
            println!("Passport first value - {}", x)
        },
        
    }
}
