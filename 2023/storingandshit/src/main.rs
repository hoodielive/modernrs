fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(32);
    v.push(75);
    v.push(89);

    let carso: &i32 = &v[2];
    println!("Yeah, thanks! {}", carso);
    
    let carso: Option<&i32> = v.get(2);
    match carso {
        Some(carso) => println!("The third element is {carso}"),
        None => println!("There is no carso."),
    }
    
}
