#![allow(dead_code)]
#![allow(unused_variables)]

use core::fmt;

enum PersonId
{
    Passport(u32),
    IdCard(u32),
}

impl fmt::Display for PersonId
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            PersonId::Passport(x) => {
                write!(f, "{}", x)
            },
            
            PersonId::IdCard(x) => {
                write!(f, "{}", x)
            }
        }
    }
}

struct Person
{
    fname: String,
    lname: String,
    id: PersonId,
}

struct Animal
{
    type_of: String,
    age: u32,
}

impl Person 
{
    fn new() -> Person
    {
        Person {
            fname: "Default".to_string(),
            lname: "Default".to_string(),
            id: PersonId::IdCard(0),
        }
    }

    fn display_info(&self)
    {
        println!("{}{} with ID {}. You are granted permission to enter the building.", &self.fname, &self.lname, &self.id,)
    }
}

impl Animal 
{
    fn new() -> Animal
    {
        Animal 
        {
            type_of: "Default".to_string(),
            age: 0,
        }
    }

}

trait DisplayInfo
{
    fn display_info(&self)
    {
        println!("{}{} with ID {}. You are granted permission to enter the building.", &self.fname, &self.lname, &self.id,)
    }
}

fn main()
{
    let mut person = Person::new();
    person.id = PersonId::IdCard(329090930);
    person.fname.push_str("Osa");
    person.lname.push_str("Meji");

    Person::display_info(&person);
}
