#![allow(unused_variables)]
#![allow(dead_code)]

struct Odu
{
    odu: String,
    direction: String,
    numbers: i32
}

impl Odu {
    new() -> &Odu;
}

fn main()
{
    let odu_ifa: Odu = Odu::new;

    {
       odu: "Oyeku Meji",
       direction: "West",
       numbers: 2
    };

    printf("This is the odu {:?}", odu_ifa.odu);

    
}
