fn unimplemented_feature()
{
    panic!("This feature is not implemented yet.");
}

fn value_processing(value: i32)
{
    match value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => panic!("Unexpected value: {}", value)
    }
}

fn main() 
{
    value_processing(44);

}
