
fn main()
{
    let mut message = String::from("Hello");
    let message_2 = &mut message;
    
    (*message_2).push_str(" World");

    println!("{}", message_2);

    let a = 10;
    let b = &a;
    let mut c = &b;
    let d = b;
    let e = &&100;

    c = e;

    println!("The address of a is: {:p}", &a);
    println!("The address of b is: {:p}", b);
    println!("The address of c is: {:p}", c);
    println!("The address of d is: {:p}", d);
    println!("The address of e is: {:p}", e);
    println!("The address of e is: {}", **e);
    println!("The address of e is: {:p}", &(**e));
    
    // Dereference
    println!("The address of c is: {:p}", *c);
    println!("The address of e is: {:p}", *e);
}
