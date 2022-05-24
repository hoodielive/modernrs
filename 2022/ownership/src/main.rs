#[warn(dead_code)]

fn rust_move()
{
    let s1: String = String::from("hello");
    let s2: String = s1.clone();

    // println!("{}, world!", s1); // This would generate an error because by default rust moved this
                                // not shallow copied it as you would expect in a language like
                                // C++ so I utilized rust's clone property.
    println!("{}, world!", s1);
}


fn main() 
{
    fn a()
    {
        let x: &str = "hello";
        let y: i32 = 22;
        b();
    }

    fn b()
    {
        let x: String = String::from("world");
    }
    
    a();

    rust_move();
}
