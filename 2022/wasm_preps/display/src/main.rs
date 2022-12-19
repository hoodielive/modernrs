#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum PersonId
{
    Passport(u32),
    IdCard(u32),
}

struct Person
{
    fname: String,
    lname: String,
    id: PersonId,
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
        println!("{}{} with ID {:?}. You are granted permission to enter the building.", self.fname, self.lname, self.id,)
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
