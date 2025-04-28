fn get_thing(x: i32) -> i32
{
    x
}

fn return_tuple() -> (char, i32)
{
    ('c', 19)
}

fn split_at(x: String) -> ()
{
    (x) 
}

fn main() 
{
    // Variables

    let trudy : i32 = 42;
    println!("{}", get_thing(trudy));

    // Tuple

    let pair = ('a', 17);
    println!("{}", pair.0);
    println!("{}", pair.1);

    // Destructuring a Tuple

    let (some_char, some_int) = ('b', 18);
    println!("{}", some_char);
    println!("{}", some_int);

    // Function return a tuple
    let middle = (1, 2);
    let (a_char, a_int) = split_at(middle);
}
