#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct User
{
    user_id: i32,
    name: String
}

#[derive(Debug)]
struct Position
{
    position: String,
    special_cred: i32,
}

fn find_user(name: &str) -> Option<i32>
{
    let name = name.to_lowercase();
    match name.as_str()
    {
        "oyeku"   => Some(1),
        "osa"  => Some(5),
        "irete" => Some(9),
        _       => None,
    }
}

fn employee_position(pos: &str) -> Option<i32>
{
    let pos = pos.to_lowercase();
    match pos.as_str() 
    {
       "administrator" => Some(1),
       "engineer"      => Some(5),
       "intern"        => Some(9),
       _               => None,
    }
}

fn main()
{
    let user_name = "Oyeku";
    let user = find_user(user_name)
                .map(|user_id| { 
                    User { 
                        user_id, 
                        name: user_name.to_owned(),
                    }
                }
            );

    match user 
    {
        Some(user) => println!("{:?}", user),
        None => println!("User not found."),
    }

    let role = "engineer";
    let id_position = employee_position(role)
                    .map(|special_cred| {
                        Position {
                            position: role.to_owned(),
                            special_cred,
                        }
                    }
                );

    match id_position
    {
        Some(role) => println!("{:?}", role),
        None       => println!("Your position is not found."),
    }


}
