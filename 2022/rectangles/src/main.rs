#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

impl Rectangle
{

    // Associated Functions

    fn area(&self) -> u32
    {
       self.width * self.height
    }

    fn square(size: u32) -> Self
    {
        Self
        {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
}

// Always borrow a struct and let '&self' retain ownership.


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

   let rect_3 = Rectangle
   {
       width: 60,
       height: 45,
   };

   Rectangle::area(&rect_1);

   println!("{:#?}", rect_1);

   println!("Can rect1 hold rect2? {}", rect_1.can_hold(&rect_2));
   println!("Can rect1 hold rect3? {}", rect_1.can_hold(&rect_3));

   let sq = Rectangle::square(3);
   println!("{:?}", sq);
}
