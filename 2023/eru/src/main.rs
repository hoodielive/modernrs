#![allow(dead_code)]
#![allow(unusued_code)]
use std::env;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
