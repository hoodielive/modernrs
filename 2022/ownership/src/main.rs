mod raiishih;
mod doitagain;
use crate::raiishih::my_mod::create_box;
use crate::doitagain::reiterate_it;

fn rust_move()
{
    let s1: String = String::from("hello");
    let s2: String = s1.clone();

    // println!("{}, world!", s1);
    // This would generate an error because by default rust moved this
    // not shallow copied it as you would expect in a language like
    // C++ so I utilized rust's clone property.

    println!("{}, world!", s1);
}


fn main() 
{
    fn a()
    {
        let x: &str = "hello";
        let y: i32 = 22;
        b();
    }

    fn b()
    {
        let x: String = String::from("world");
    }

    a();

    rust_move();

    // Allocate integer on heap.
    let _box2 = Box::new(5i32);

    // A nested scope:

    {
        // Allocate an integer on the heap

        let _box3 = Box::new(4i32);

        // _box3 is destroyed here, and memory gets freed.
    }

    // Creating lots of boxes just for fun.
    // There's no need to manually free memory.

    for _ in 0u32..1_000
    {
        create_box();
    }

    reiterate_it();
}
