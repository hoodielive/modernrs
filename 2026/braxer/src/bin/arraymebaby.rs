#![allow(unused)]

fn main()
{
    // Array - fixed length, known at compile time.    
    let arr: [u32; 3] = [ 1, 2, 3 ];

    println!("{}", arr[2]);
    
    let arr2: [u32; 10] = [0; 10];
    println!("{}", arr2[2]);
    
    // Slice - length not known at compile time.


}
