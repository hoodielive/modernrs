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
    println!("{:?}", weather)
}
