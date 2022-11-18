#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

// Always borrow a struct and let '&self' retain ownership.

fn area(ret: &Rectangle) -> u32
{
   ret.width * ret.height
}

fn main()
{
   let rect_1 = Rectangle
   {
       width: 30,
       height: 50,
   };

   let scale = 2;
   let rect_2 = Rectangle
   {
       width: dbg!(30 * scale),
       height: 50
   };

   area(&rect_1);

   println!("{:#?}", rect_1);
}
