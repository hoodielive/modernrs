#![allow(unused_variables)]
#![allow(unused_mut)]


fn main() 
{
    let mut f = String::new();
    let mut g = String::from("foo");
    let data = "initial contents";
    let c = "intitial contents".to_string();
    let s = data.to_string();
    
    f.push_str("bar");
    g.push_str("bar");
    
    println!("f contains: {}", f);
    println!("g contains: {}", g);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);

    let mut p = String::from("lo");
    p.push('l');
    println!("p is {}", p);
    
    let p1 = String::from("tic");
    let p2 = String::from("tac");
    let p3 = String::from("toe");

    println!("p3 is {}", p3);

    let p4 = format!("{}-{}-{}", p1, p2, p3);
    println!("{} is the message.", p4);
    
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
}
