const A_VAL: i32 = 30 * 30 * 30;

fn main() {
    let x = 5;
    // shadowing
    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner: The value of x in the inner scope is: {x}");
    }

    println!("Outer: The value of x in the inner scope is: {x}");
}
