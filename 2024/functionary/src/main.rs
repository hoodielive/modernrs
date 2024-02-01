fn main() 
{
    let _numbers: Vec<i32> = (1..)
        .map(|x| x + 1)
        .map(|x| x * x)
        .take(5)
        .collect();

    println!("{:?}", _numbers);

    let _urgency: Vec<i32> = vec![1, 2, 3];

    let _plus_one = _numbers.iter().map(|x| x + 1);
    _plus_one.for_each(|x| println!("{}", x));

    let _larger_then_two = _numbers.into_iter().filter(|&x| x > 2);
    _larger_then_two.for_each(|x| println!("{}", x));
}
