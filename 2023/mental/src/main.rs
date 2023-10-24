#![allow(dead_code)]
#![allow(unused_variables)]


enum Mind
{
    Rere,
    Pele,
    Buruku
}


struct MindTypes
{
    Ori: String,
}

impl MindTypes
{
    fn new() -> &Self
    {
       return &Self 
    }
}

fn main()
{
    protocol: MindTypes = MindTypes::new;    
    println!(protocol);
}
