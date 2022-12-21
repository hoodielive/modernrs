fn main() 
{
    let x: () = ();
    let y: () = println!("Hello, World!");
    assert_eq!(x, y);
    println!("All units are the same!");
}
