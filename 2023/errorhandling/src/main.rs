fn divide(x: i32, y: i32) -> Result<i32, String>
{
    if y == 0 
    {
        Err("cannot divide by zero".to_string())
    } 
    else 
    {
        Ok(x/y)
    }
}
fn main()
{
    let result = divide(10, 2);

    match result 
    {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
