fn say_hello() {
    println!("hello");
}

pub fn closures() {
    let sh = say_hello;
    sh();

    // Closure.
    let mut two = 2;
    {
        let plus_two = |x: i32| {
            let mut z = x;
            z += two;
            z
        };
    }
    let plus_one = |x: i32| -> i32 { x + 1 };
    let a = 6;

    println!("This {} + 1 = {}.", a, plus_one(a));

    let plus_two = |x| {
        let mut z = x;
        z += 2;
        z
    };

    println!("{} + 2 = {}", 3, plus_two(3));

    let borrow_two = &mut two;

    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}
