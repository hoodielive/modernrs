// Component
#[derive(Debug)]
struct Player;

impl Player {
    fn new(&self) -> Player {
        &self        
    }

}

fn main()
{
    let player1: Player = Player;
    println!("What is this {:?}?", player1)
    
}
