use std::{fs::File, path::Path};

struct Car { }

fn memory_example()
{
    // car is the owner of the heap 
    let car = Box::new(Car {}); // allocate memory on the heap 
                                // if you need a shared mem alloc
                                // use Rc::new(Car {}) and then 
                                // let car2 = car.clone();
    let car2 = car; // car2 is now owner
    let my_string = String::from("Oji"); // allocate memory on heap
    function_that_can_panic(); // car and my_string goes out of scope and mem cleaned up
    if !should_continue() { return; }
}

fn file_example()
{
    let path = Path::new("example.txt");
    let file = File::open(&path).unwrap();
    function_that_can_panic();
    if !should_continue() { return; }
}

fn main() 
{
}
