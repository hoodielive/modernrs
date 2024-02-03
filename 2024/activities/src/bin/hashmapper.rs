#![allow(dead_code)]
use std::collections::HashMap;

fn main()
{
        
}

struct Inventory
{
    chairs: i8,
    beds: i8,
    tables: i8,
    coches: i8,
}

fn print_inventory(inventory: &Inventory)->Result<(), String>
{
   let _the_inventory: HashMap<&Inventory, i8>; 

   _the_inventory.insert(Inventory::chairs, 2);
   _the_inventory.insert(Inventory::beds, 0);
   _the_inventory.insert(Inventory::tables, 3);
   _the_inventory.insert(Inventory::couches, 1);

   match inventory 
   {
      Inventory::chairs => Ok(()),
      _ => Err(String::from("You are fresh out of luck")),
   }
}
