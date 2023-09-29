#[derive(Debug)]
enum Holidays {
    Kwanza,
    Juneteenth,
    IfaFestival,
}

fn inspect(day: Holidays) -> String {
    match day {
        Holidays::Kwanza      => String::from("October 2"),
        Holidays::Juneteenth  => String::from("June 19"),
        Holidays::IfaFestival => String::from("October 9")
    }
}

fn main() {
    let day = Holidays::Kwanza;
    let date = inspect(day);
    println!("{:?}", date);
}
