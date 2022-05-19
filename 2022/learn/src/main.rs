#![allow(unused_variables)]
#![allow(dead_code)]
mod buildwithrust;
mod strings;

// Functions.

fn reverse(pair: (i32, bool)) -> (bool, i32) 
{
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn materialize_stuff() {
    new_function();
    show_off(21);
}

fn show_off(x: i32) -> i32 {
    println!("Hello {}", x);
    20
}

fn new_function()
{
    // println is a macro (segment of code that is given a name)
    println!("Hello");
}

fn fun_como()
{
    let location: (&str, f64, f64) = ("KCLE", 41.4094069, -81.8546911);
    let (name, latitude, longitude) = location;
    println!("Location name: {}, latitude: {}, longitude: {}", name, latitude, longitude);
}

fn mutation_and_shihh()
{
    let mut name: &str = "Jon Doe";

    println!("{}", name);
}

fn say_hello(name: &mut &str)
{
    println!("Hello {}", name);
    *name = "Jane Doe";
}
fn primitives_() 
{
    // 8-bit integers 
    // 2's compliment 
    // Unsigned 8-bit binary (u)
    // Signed 8-bit binary (i)
    // -127 - 128

    let abool: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    let an_int2: i32 = 5;
    let default_float = 3.0;
    let default_integer = 7;

    // Mutables
    let mut _mutable = 12; // i32
    _mutable = 21;

    println!("The integer is {}", an_int2);
    println!("The integer is {}", an_integer);

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability
    println!("1 Million is written as {}", 1_000_000u32);

}

fn strice()
{
    let person_name_string: String = String::from("Osa");
    let person_name_slice: &String = &person_name_string;
    let person_name_slice2: &str = person_name_string.as_str();
    let duck = "Duck";
    let dog = "airlines";
    let airline_name = format!("{}{}", duck, dog);
    println!("{}", airline_name);

    let mut slogan = String::new();
    slogan.push_str("We hit the ground!"); 
    slogan.push(' ');
}



#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);


fn main() {
    materialize_stuff();
    primitives_();
    strice();
    buildwithrust::funfunfun();
    let greeting = strings::Greeting::new("Osa");
    println!("Hallo!, {}", greeting.name);
}
