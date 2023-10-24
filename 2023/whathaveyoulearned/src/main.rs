// Component
#[derive(Debug)]
struct Player{
   active: bool, 
}

impl Player {
    fn new() -> Player {
        Player { active: true }
    }

}

fn main()
{
    let player1: Player = Player { active: false };
    println!("What is this {:?}?", player1);
    
}
