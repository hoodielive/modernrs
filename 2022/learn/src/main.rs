fn reverse(pair: (i32, bool)) -> (bool, i32) 
{
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);


fn main() {

    let _abool: bool = true;
    let _a_float: f64 = 1.0;
    let _an_integer = 5i32;
    let _an_int2: i32 = 5;
    let _default_float = 3.0;
    let _default_integer = 7;

    // Mutables
    let mut _mutable = 12; // i32
    _mutable = 21;


    println!("The integer is {}", _an_int2);
    println!("The integer is {}", _an_integer);

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability
    println!("1 Million is written as {}", 1_000_000u32);
}
