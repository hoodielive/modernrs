pub mod greet 
{
    pub fn hello()
    {
        println!("Hello");
    }

    pub fn goodbye()
    {
        println!("Goodbye");
    }

}

pub mod math 
{
    pub fn add(a: i32, b: i32) -> i32
    {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32
    {
        a - b
    }
}

fn main()
{
    use greet::*;
    hello();

    use math::*;
    let result = add(3, 4);
    let result2 = sub(4, 3);
    println!("{:?}", result);
    println!("{:?}", result2);
}
