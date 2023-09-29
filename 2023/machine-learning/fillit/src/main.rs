#![feature(core_intrinsics)]
fn print_type_of<T>(_: &T) {
   println!("{}", { 
       std::intrinsics::type_name::<T>() 
   });
}

fn square_of(x: i32) -> i32 {
    println!("x = {:?}", x);
    x.pow(2)
}

fn main()
{
    let x = "learning rust";
    let y = 6;
    let z = 3.14;
    
    println!("{}", x);
    print_type_of(&x);
    print_type_of(&y);
    print_type_of(&z);

    let mut xa = 32;
    println!("Mutability works like: {}", xa);
    xa = 64;
    println!("And then...");
    println!("like: {}", xa);

    square_of(y);

    let place = "himalayas";
    
    // Rust's if statements can return values!
    let weather = if place == "himalayas" {
        "cold"
    } else {
        "hot"
    };
    println!("{:?}", weather);

    let place2 = "himalayas";

    // Same with pattern-matching :: match
    let weather2 = match place2 {
        "himalayas" => "cold",
        _ => "hot",
    };
    println!("{:?}, was chosen", weather2);


    let lang = String::from("rust");
    let rust1 = add_version(&lang); // You can't borrow a string and then mutate it.
    println!("{:?}", rust1);
    let rust2 = add_lang(&lang);
    println!("{:?}", rust2);

    
}

fn add_version(s: &str) -> String {
    s.to_string() + " 2021."
}

fn add_lang(s: &String) -> String {
    s.to_string() + " lang."
}

#[test]
fn test_add_version() {
   assert_eq!(add_version("abcd"), String::from("abcd 2018."));
}
