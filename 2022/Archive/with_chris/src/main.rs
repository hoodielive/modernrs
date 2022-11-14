#![allow(dead_code)]
#![allow(unused_variables)]

mod new_impl;

struct Point
{
    x: T,
    y: T,
}

impl <T> Point<T>
{
    fn get_x(&self) -> &T
    {
        &self.x;
    }
}

impl <T> Point<f32>
{
    fn distance_from_origin(&self) -> f32
    {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {

    let p = Point { x: 5, y: 10 };
    p.distance_from_origin();

    let fp = Point { x: 5.2, y: 10.5 };
    fp.distance_from_origin();

    println!("p.x = {}", p.get_x());

    // From new_impl.rs
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.new_impl::mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
