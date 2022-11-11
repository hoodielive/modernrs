#![allow(unused_variables)]
#![allow(dead_code)]

struct Discount
{
    Percent(i32),
    Flat(i32),
}

struct Ticket 
{
    event: String,
    price: i32,
}

struct Book
{
    quotes: String,
}

fn display_quotes(quote: Book)
{
    println!("{}", quote);
}

fn main()
{
   let aquote = Book {
        quotes: String::from("This is a string."),
    };

    // TODO This won't work. Why?
    // display_quotes(aquote);

    let n = 3;
    match n {
        3 => println!("Three."),
        other => println!("Number: {:?}.", other),
    }

    let flat = Discount::Flat(2);
    match flat
    {
        Discount::Flat(2) => println!("Flat 2."),
        Discount::Flat(amount) => println!("Flat discount of {:?}.", amount),
        _ => (),
    }

    let concert = Ticket {
        event: "Concert".to_owned(),
        price: 50.0,
    }

    match concert {
        Ticket { price, .. } => println!("Price = {:?}", price), // .. means ignore anything else
    }

}
