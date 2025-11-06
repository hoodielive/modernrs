#![allow(dead_code)]
#![allow(unused_variables)]

#![derive(Debug)]
struct User
{
    active: bool, 
    username: String,
    email: String,
    sign_in_count: u64,
}

#![derive(Debug)]
struct BJ
{
    active: bool, 
    _float: f64,
    _speed: f64
}

fn main() 
{
   let user1 = User
   {
       active: true,
       username: "Freedie Mac".to_string(),
       email: "bloodymerry@gstring.com".to_string(),
       sign_in_count: 100,
   };

   let user_bj = BJ
   {
    active: true,    
    _float: 3.4,
    _speed: 66.6
   };

   println!("This is the information for a User {}", user1);
   println!("The query is {:?}", user_bj);
}
