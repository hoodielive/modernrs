#![allow(dead_code)]

struct Rectangle
{
    width: u32,
    height: u32,
}

fn return_rect(ret: Rectangle) -> Rectangle
{
   ret 
}

fn main()
{
   let values: Rectangle = Rectangle { width: 500, height: 600 };
   return_rect(values);
}
