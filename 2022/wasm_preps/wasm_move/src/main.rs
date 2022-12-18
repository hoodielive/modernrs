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

// Unit Struct
struct Animal(String, u32, String);

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
    
    let animal = Animal("dog".to_string(), 10, "bulldog".to_string());
    let animal2 = Animal(String::from("cat"), 13, String::from("Simese"));
    let Animal(animal_type, animal_num, animal_name) = animal2;
    println!("{} {} {}", animal_type, animal_num, animal_name);
}

fn check_person_id(id: PersonID)
{
    if let PersonID::IdCard(num) = id
    {
       println!("It is matching id {}", num);
    }
    else
    {
        println!("It doesn't match!");
    }
        
    match id
    {
        PersonID::IdCard(x) =>
        {
            println!("ID card: first value - {}", x)
        },
        PersonID::Passport(x, y, z) => {
            println!("Passport first value - {}", x)
        },
        
    };
}
