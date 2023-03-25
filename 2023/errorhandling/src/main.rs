trait Printable
{
    fn print(&self);
}

impl Printable for i32
{
    fn print(&self)
    {
        println!("This is a string: {} ", *self);
    }
}

impl Printable for String
{
    fn print(&self)
    {
        println!("This is a string: {} ", *self);
    }
}

fn print_item<T: Printable>(item: T)
{
    item.print();
}

fn main()
{
    
    let a = 42;
    let b = "howdy".to_string();

    print_item(a);
    print_item(b);
}
