trait Bite {
    fn bite(self: &mut Self) {}
}

#[derive(Debug)]
struct Grapes {
    grapes_left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.grapes_left -= 1;
    }
}

fn main() {}
