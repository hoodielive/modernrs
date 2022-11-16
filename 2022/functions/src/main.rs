fn main() 
{
    another_function(5, 'h');

    let x = {
        let y = 3;
        y + 3
    };

    println!("{:?}", x);

    let example = marvel(255);
    println!("{}", example);
}

fn marvel(a: i32) -> i32
{
    a
}


fn another_function(value: i32, unit_label: char) 
{
    println!("I will be there in {value}{unit_label}");
}
