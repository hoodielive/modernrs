#![allow(unused_variables)]

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

    println!("The home address is of {}, and the address is {}", 
                home.kind,
                home.address
            );
}

fn route(ip_kind: IpAddrKind) {}
