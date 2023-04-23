#[allow(dead_code)]
pub struct GrayscaleMap
{
    pub pixels: Vec<u8>,
    pub size: (usize, usize),
}

struct broom
{
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }

fn main() 
{
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height)
    };

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap
{
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

fn chop(b: Broom) -> (Broom, Broom)
{
    let mut broom1 = Broom { height: b.height / 2, .. b};
    let mut broom2 = Broom { name: broom1.name.clone() /  .. broom1 };
    
}
