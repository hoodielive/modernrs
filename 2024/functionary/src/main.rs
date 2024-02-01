fn main() 
{
    let _numbers: Vec<i32> = (1..)
        .map(|x| x + 1)
        .map(|x| x * x)
        .take(5)
        .collect();

    println!("{:?}", _numbers);
}
