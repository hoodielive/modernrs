fn main() {
     let rect1 = (10, 50);
     println!("This is the areas {}.", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32
{
    dimensions.0 * dimensions.1
}
