


fn main() 
{
    let mut s = String::from("Hello ");
    {
        let r1 = &mut s;
        change(r1)
    }
    let r2 = &mut s;

    change(r2);

    let reference_to_nothing = dangle();
}

fn change(some_string: &mut String)
{
    some_string.push_str(", world");
    println!("Some_string: {:?}.", some_string);
}

fn dangle() -> &String 
{
    let s = String::from("hello");
    &s
}
