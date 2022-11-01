#![allow(unused_variables)]

#[derive(Debug)]
struct Bucket
{
    liters: u32,
}

fn pour(source: &Bucket, target: &Bucket, amount: u32)
{
    source.liters -= amount;
    source.liters += amount;
}

fn main()
{
    let mut bucket1 = Bucket { liters: 20 };
    let mut bucket2 = Bucket { liters: 10 };

    pour(&bucket1, &bucket2, 3);

    println!("Bucket 1: {:?}.", bucket1);
    println!("Bucket 2: {:?}.", bucket2);
}
