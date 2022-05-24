mod controls;

// functions
fn my_function(x: i32, y: i32) -> i32
{
    println!("Another function. {x}");
    println!("Another function. {y}");
    x + y
}


fn main() 
{
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);

    x = 6;

    println!("The value of x is: {}", x);

    // You cannot mutate (mut) a constant.
    // They must also be annotated. (:u32)

    const _SUBSCRIBER_COUNT: u32 = 100_000;

    // Scalar datatypes (represents single value and there are 4 of them):
    // integers, floats, booleans and chars.
    // So rust defaults integers to i32.

    let _a = 98_222; // which is to say, a: i32 = 98_222

    // Compound types:
    // tuples, arrays

    // Tuples:
    let tup: (&str, i32) = ("Lets get busy", 100_000);

    // destruct tuple
    let (_channel, _sub_count) = tup;
    let _sub_count: i32 = tup.1;

    // Arrays:
    let _error_codes = [401, 404, 500];

    // Create an array with 8 values all assigned to 0.
    // let byte: [i32; ] = [0; 8];

    my_function(32, 12);
    controls::a_forloop();
    controls::a_control();
}
