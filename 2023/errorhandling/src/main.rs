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

struct Mainer
{
    insta: String,
    virtpost: i32,
}

impl Mainer
{
    fn constructor(&self)
    {
        println!("go public {} ", *self);
    }
}

fn showConstructor<T: Mainer>(val: String) 
{
    val.constructor();
}

fn main()
{
    let a = 42;
    let b = "howdy".to_string();

    print_item(a);
    print_item(b);
}
