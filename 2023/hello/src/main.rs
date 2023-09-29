use hello::greet;

struct RedFox {
    enemy: bool,
    life: i32,
}

impl RedFox {
    fn new() -> RedFox {
        RedFox {
            enemy: true,
            life: 70,
        }
    }
}

trait Run {
    fn run(&self) {
        println!("I'm running!");
    }
}

struct Robot {}
impl Run for Robot {}

fn main() {
    let mut s1 = String::from("abc");
    do_stuff(&mut s1);

    let info: (i32, f32, f32) = (1, 3.3, 999.0);
    let (jets, fuel, ammo) = info;
    greet();

    println!("The first one is {jets}, the second one is {fuel}, the third one is {ammo}");

    let julius: RedFox = RedFox {
        enemy: false,
        life: 80,
    };

    println!("His name is {}", julius.enemy);

    let mut fox = RedFox::new();
    fox.enemy = false;

    let life_left = fox.life;

    let robot = Robot {};
    robot.run();
}

fn do_stuff(s: &mut String) {
    s.insert_str(0, "Hi, ")
}
