#![allow(dead_code)]
#![allow(unusued_variables)]

struct Point<T>
{
    x: T,
    y: T,
}

fn take_over<T: std::cmp::PartialOrd>(list: &[T]) -> &T
{
    &T
}

fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T
{
    let mut largest = &list[0];

    for item in list
    {
        if item > largest
        {
            largest = item;
        }
    }

    largest
}

fn main() 
{
   let number_list = vec![34, 50, 25, 100, 65];

   let result = largest_generic(&number_list);
   println!("The largest number is {}.", result);

   let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
   let result = largest_generic(&number_list);
   println!("The largest number is {}.", result);
}
