
/* The Borrowing law
 * One mutable reference 
 * or
 * Any number of immutable references
 * But never both
 * */

fn main()
{
    let mut s = String::from("Rust");
    
    let r1 = &s;
    let r2 = &s;

    println!("{}", r1);
    println!("{}", r2);

    let r3 = &mut s;
    r3.push_str("!");
    
    println!("{}", r3);
}
