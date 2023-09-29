#![allow(unused_variables)]

mod re;

fn main() 
{
    let s: String = String::from("Hello World");
    let hello: &str = &s[..5];
    let world: &str = &s[..];
    let word: &str = first_word(&s);

    let b: String = String::from("So sophisticated.");
    re::reimplement(&b);

    let s1 = String::from("howdy");
    println!("The first 2 please: {}", &s1[..2]);
    println!("From 3 onward is: {}", &s1[3..]);
    println!("And the entire string is: {}", &s1[..]);
}

fn first_word(s: &String) -> &str
{
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() 
    {
        if item == b' '
        {
            return &s[..i];
        }
    }
    &s[..]
}
