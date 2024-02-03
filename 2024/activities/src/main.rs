#![allow(dead_code)]

fn main() 
{
   let _manager = Employee
   {
       position: Position::Manager,
       status: Status::Terminated,
   };

   match display_access(&_manager)
   {
        Err(e) => println!("Access is denied: {:?}", e),
        _ => (),
   }
}

enum Position 
{
    Maintenance,
    Marketing,
    Manager,
    LineSupervisors,
    KitchenStaff,
    AssemblyTechnicians
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
        Status::Terminated => 
          return Err("You have been terminated. You know you don't have access right?".to_owned()),  
           _ => ()
    }
    
    match employee.position
    {
        Position::Manager => Ok(()), 
        Position::Marketing => Ok(()), 
        Position::Maintenance => Ok(()), 
        _ => Err("You cannot enter this building and you know it".to_owned()),
    }
}

fn display_access(employee: &Employee)->Result<(), String>
{
   let _has_access = building_access(employee);
   println!("Has access.");
   Ok(())
}
