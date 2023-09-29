#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
enum IpAddrKind
{
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr
{
    kind: IpAddrKind,
    address: String,
}

enum Message
{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message
{
    // use &self to get the value that we called the method on.
    fn call(&self) {}
}

enum Option<T>
{
    None,
    Some(T),
}

enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState
{
    Alabama,
    Georgia,
    Pennsylvania,
    Florida,
    Texas,
    Arizona,
    Illinois,
    Washington
}

fn value_in_cents(coin: Coin) -> u8
{
    match coin
    {
        Coin::Penny => {
            println!("Lucky penny found!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("I found this quarter in {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>
{
    match x 
    {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main()
{
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr
    {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr
    {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("The home address is of {:?}, and the address is {:?}", home.kind, home.address);
    let m = Message::Write(String::from("I am here."));
    m.call(); 

    let some_number = Some(5);
    let some_char = Some('c');
   // let absent_number: Option<i32> = None;
    
    let some_other = Some(5).is_some();

    if some_other == true
    {
        println!("I have the gift of protech.");
    }
    println!("Do you work {:?}", some_other);

    let newcoin: Coin = Coin::Quarter(UsState::Georgia);
    println!("The value is {:?}",value_in_cents(newcoin));

   let five = Some(5);
}

fn route(ip_kind: IpAddrKind) {}
