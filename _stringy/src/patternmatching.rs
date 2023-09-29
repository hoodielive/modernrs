fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        z @ 9..11 => "lots of oranges",
        _ if (x % 2 == 0) => "some",
        _ => "a few",
    }
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3, 4);

    match (point) {
        (0, 0) => println!("Origin."),
        (0, y) => println!("x axis, y = {}", y),
        (ref mut x, 0) => println!("the point is somewhere on the y axis, x = {}", x),
        (_, y) => println!("(?,{})", y),
    }
}

fn main() {
    pattern_matching();
}
