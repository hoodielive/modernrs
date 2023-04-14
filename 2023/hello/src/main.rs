use hello::greet;

fn main() {
    let info: (i32, f32, f32) = (1, 3.3, 999.0);
    let (jets, fuel, ammo) = info;
    greet();

    println!("The first one is {jets}, the second one is {fuel}, the third one is {ammo}");
}
