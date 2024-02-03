use std::collections::HashMap;

fn main() 
{
    let mut people = HashMap::new();

    people.insert("Ose", 33);
    people.insert("Osa", 83);
    people.insert("Oyeku", 24);
    people.insert("Ogunda", 39);

    match people.get("Soba")
    {
        Some(age) => println!("Age is: {:?}", age),
        None => println!("not found"),
    }
}
