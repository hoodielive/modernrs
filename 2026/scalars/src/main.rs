#![allow(unused)]

fn main() 
{
     let i0: isize = 1; 
     let u0: usize = 1;

     let i1: i8 = 1; 
     let u1: u8 = 1;

     // Type conversion

     let iq: i32 = 1; 
     let uq: u32 = iq as u32; 
     let x: u32 = uq + (iq as u32);

    // Min and max

     let min_i: i32 = i32::MIN;
     let max_i: i32 = i32::MAX;

    // Character

    let e: char = '🦀';

    println!("Let me ask what this is {}", e);

    println!("i32 min: {min_i}");
    println!("i32 max: {max_i}");


    let lang = "rust";
    println!("hello {} {}", lang, lang);

    let nmod = asychron();

    println!("{}", nmod())
}
 
fn asychron()
{
    println!("Just a test")
}
