fn main()
{
    // s1 (pointer)
    let s1 = String::from("Rust"); // heap allocated string
    // // let s2 = s1.clone();
    // let s2 = &s1;

    print_string(s1.clone());
    println!("s1 is : {s1}");
    // s1 is dropped
    
    let s3 = generate_string();
    
}

fn add_to_string(mut p1: String) -> String
{
    p1.push_str(" is Awesome sauce!");
    p1
}

fn generate_string() -> String
{
   String::from("Ferris")
}

fn print_string(p1: String) 
{
    println!("{p1}")
}
