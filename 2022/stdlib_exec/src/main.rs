use std::mem;

#[derive(Debug, Clone, Copy)]
struct Point
{
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle
{
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point
{
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point>
{
    // Allocate the type annotations are superfluous 
    // Stack allocated variables
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() 
{

    #[allow(unused_variables)]
    let point: Point = origin();

    #[allow(unused_variables)]
    let rectangle: Rectangle = Rectangle { top_left: origin(), bottom_right: Point { x: 3.0, y: -4.0 }};

    #[allow(unused_variables)]
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle { top_left: origin(), bottom_right: Point { x: 3.0, y: -4.0 } });
}

