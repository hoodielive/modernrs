use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static!
{
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> =
    {
        let mut map = HashMap::new();
        map.insert("Osa", vec!["user", "admin"]);
        map.insert("Oyeku", vec!["user"]);
    };
}

fn show_access(name: &str)
{
    let access = PRIVILEGES.get(name);
    println!("{}: {:?}", name, access);
}

fn main() 
{
    let access = PRIVILEGES.get("Osa");
    println!("Osa: {:?}", access);

    show_access("Oyeku");
}
