#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;


enum Bills
{
    Add,
    View,
    Remove,
    Update,
    Total
}

fn bill_orientation()
{
    println!("== Manage Bills ==");
    println!("1. Add bill");
    println!("2. View bills");
    println!("3. Remove bill");
    println!("4. Update bill");
    println!("5. Bill total.");
    println!("Enter a selection: ");
    
    //let mut buffer = String::new();
    //io::stdin().read_line(&mut buffer)?;
    //Ok(buffer.trim().to_owned()) 
}

fn determine_management_vector(choice: String) -> String
{
        match &choice
        {
            Add => Bills::Add,
            View => Bills::View,
            Remove => Bills::Remove,
            Update => Bills::Update,
            Total => Bills::Total,
        };
    choice
}

fn main() 
{
    bill_orientation();
    let choice = determine_management_vector("Add".to_owned());
    
    match determine_management_vector(choice)
    {
        Add => println!("Add Bill"),
        View => println!("View Bill"),
        Remove => println!("Remove Bill"),
        Update => println!("Update Bill"),
        Total => println!("Total Bill"),
    };
}
