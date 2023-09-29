#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

#[derive(Debug)]
enum IPV
{
    ipv4(u16, u8, u8, u8),
    ipv6(String)
}


fn main()
{
    // i32 is inferred from values

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("The third element of v is {}", third);

    let ipv4_structure: IPV = IPV::ipv4(256, 0, 0, 1);
    println!("The ipv4 address for localhost is {:?}", ipv4_structure);

    let ipv6_structure: IPV = IPV::ipv6(String::from("::64"));
    println!("The ipv6 address for localhost is {:?}", ipv6_structure);

    // Another way to access the index of a Vector

    let third: Option<&i32> = v.get(9);

    match third
    {
        Some(third) => println!("The third element is {}.", third),
        None => println!("There is no third element."),
    }
}
