trait Printable 
{
    fn print(&self);
}

impl Printable for i32
{
    fn print(&self)
    {
        println!("Printing i32: {}", *self);
    }
}

impl Printable for String 
{
    fn print(&self)
    {
        println!("Printing for String: {}", *self);
    }
}

fn print_item<T: Printable>(item: T)
{
    item.print();
}


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

    let a = 42;
    let b = "hello".to_string();
    print_item(a);
    print_item(b);
}
