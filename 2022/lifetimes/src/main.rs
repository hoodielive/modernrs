#![allow(dead_code)]
#![allow(unused_variables)]

fn create_model<T>(life: &[T]) -> String
{
//    println!("{} is the answer that has been chosen." &T);
    unimplemented!()
}

pub fn print_type_of<T>(_: &T)
{
    println!("{}", std::any::type_name::<T>())
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
{
    if x.len() > y.len()
    {
        x
    } else {
        y
    }
}

fn main()
{
    let r;
    let x = 5;
    r = &x;

    println!("r: {}", r);
    //create_model(r);
    
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
