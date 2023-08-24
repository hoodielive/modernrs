fn main()
{
    let mut string1: String = String::new();
    string1 = "A string".to_string();

    let string2 = &string1;

    println!("{:?}", string1);

    println!("{:?}", stuff(5, 6));
}

fn stuff(x: i32, y: i32) -> i32
{        
    x * y
}
