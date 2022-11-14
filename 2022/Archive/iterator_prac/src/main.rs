#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unsafe_code)]

mod inline_mods;
mod new_inline_mods;

enum Color
{
    Red,
    Blue,
    Green
}

fn main() 
{
    let data: Vec<_> = vec![1, 2, 3, 4, 5]
                        .iter()
                        .map(|num| num * 3)
                        .filter(|num| num > &10)
                        .collect();
    for num in data
    {
        println!("{:?}", num);
    }

    let range = 1..=3;
    let range = 1..4;

    for num in 1..4 {
        println!("{:?}", num)
    }

    for ch in 'a'..='f' {
        println!("{:?}", ch)
    }

    let maybe_user = Some("jerry");

    match maybe_user {
        Some(user) => println!("user={:?}", user),
        None => println!("no user"),
    }

    if let Some(user) = maybe_user {
        println!("user={:?}", user);
    } else {
        println!("no user.");
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("Its red.");
    } else {
        println!("Its not red.");
    }

    // While let
    let mut data = Some(3);

    while let Some(i) = data {
        println!("loop");
        data = None;
    }
    println!("done");

    let numbers = vec![1, 2, 3];
    let mut number_iter = numbers.iter();
    while let Some(num) = number_iter.next() {
        println!("Num = {:?}", num);
    }

    println!("done");

    new_inline_mods:: b
}
