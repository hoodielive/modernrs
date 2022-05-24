pub fn a_control()
{
    let mut number: i32 = 3;

    while number != 0
    {
        println!("{}!", number);
        number -= 1;
    }

    println!("LiftOff!!!");

}

pub fn a_forloop()
{
    let a = [10, 20, 30, 40, 50];

    for element in a.iter()
    {
        println!("The value is: {}", element);
    }

    for number in 1..4
    {
        println!("{}!", number)
    }
}
