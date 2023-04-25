#[allow(unused_variables)]
#[allow(dead_code)]
pub struct GrayscaleMap
{
    pub pixels: Vec<u8>,
    pub size: (usize, usize),
}

#[allow(dead_code)]
struct Broom
{
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }

fn main() 
{
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height))
    };

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };

    #[allow(unused_variables)]
    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
}

#[allow(dead_code)]
fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap
{
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

fn chop(b: Broom) -> (Broom, Broom)
{
    let mut broom1 = Broom { height: b.height / 2, .. b};
    let mut broom2 = Broom { name: broom1.name.clone(), .. broom1 };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}
