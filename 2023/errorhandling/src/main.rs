trait Printable 
{
    fn print(&self);
}

impl Printable for i32
{
    fn print(&self)
    {
        println!("You will return an int32: {} ", *self);
    }
}

impl Printable for String 
{
    fn print(&self)
    {
        println!("You will return a String called: {} ", *self);
    }
}

fn print_stuff<T: Printable>(item: T)
{
    item.print();
}

fn main()
{
    let a = 42;
    let b = "helllllooooo".to_string();
    print_stuff(a);
    print_stuff(b);
}
