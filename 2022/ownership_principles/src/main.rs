#![allow(unused_variables)]
#![allow(dead_code)]

fn main()
{
    let s = String::from("hello");
    
    takes_ownership(s);

    let x = 5;

    makes_copy(x);


    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(&s2);

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}

fn takes_ownership(some_string: String)
{
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32)
{
    println!("{}", some_integer)
}

fn gives_ownership() -> String
{
   let some_string = String::from("Yours");
   some_string
}

fn takes_and_gives_back(a_string: &String) -> &String
{
   a_string 
}
