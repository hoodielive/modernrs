use std::any::type_name;

// typeOf implementation
fn type_of<T>(_:&T) -> &'static str
{
    type_name::<T>()
}

fn speed(start: u32, end: u32, time_elasped: u32) -> u32 
{
    let distance = end - start;
    distance / time_elasped
}

fn main()
{
  let b: u8 = 100;
  let a: u32 = b.into();
  println!("This is b: {} and this is a: {}", b, a);

  let _x = 42; 
  let _y: u32 = _x;

  println!("The type of x is: {}", type_of(&_x));
  let _speed = speed(30, 90, 4);
  println!("{} is what all that balls down to.", _speed);
  println!("{:?}", 1..5);

  let end = 5;
  let mut sum = 0;

  for i in 1..(end + 1)
    {
        sum += i;
    }
}
