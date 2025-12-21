
#[allow(dead_code)]
fn add2(x: i32, y: i32) -> i32
{
    // Implicit return (no semicolon)
    x + y
}

#[allow(unused_variables)]
//#[allow(unused_assignments)]
fn main() 
{
    // Immutable binding
    // All variables are immutable unless explicitly stated by using the 'mut' keyword.
    let x: i32 = 3;
    println!("The value of x is: {}", x);

    // Integer/float suffixes
    // When would you use a float as opposed to an integer
    let y: i32 = 13i32;
    let f: f64 = 1.3f64; // Precision
    
    // Type inference
    let implicit_x = 1;
    let implicit_f: f64 = 1.3f64;

    // Arithmetic
    // Rust does not allow you to write bad code.
    let sum_of = x + y + 44;

    // Mutable variable 

    let mut mutable = 1;
    mutable = 4;
    mutable += 2;
    println!("{}", mutable);

    // String literals
    let p: &str = "hello, Chris";
    println!("{}", p);

}

