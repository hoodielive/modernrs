#![allow(dead_code)]
fn main() 
{
    let _manager = Employee
    {
        position: Position::Manager,
        status: Status::Terminated,
    };

    match print_access(&_manager)
    {
        Err(e) => println!("Access denied: {:?}", e),
        _ => (),
    }

}

enum Position
{
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech
}

struct Employee
{
    position: Position,
    status: Status,
}

enum Status
{
    Active,
    Terminated
}

fn building_access(employee: &Employee)->Result<(), String>
{
    match employee.status
    {
       Status::Terminated => return Err("You are terminated. Entry is denied.".to_owned()),
           _ => (),
    }

    match employee.position
    {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Invalid position".to_owned()),
    }
}


fn print_access(employee: &Employee)->Result<(), String>
{
   let _attempt_access = building_access(employee)?;
   println!("Accepted: ");
   Ok(())
   
}
