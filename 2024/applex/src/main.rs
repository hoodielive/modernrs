#[allow(dead_code)]

#[derive(Debug)]
struct Celibate
{
    active: bool,
    inactive: bool, 
}

fn main() 
{
    let state = Celibate { active: true, inactive: false };
    println!("The principle is: {:?}", state);
}
