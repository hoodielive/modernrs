#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Deck
{
    cards: Vec<String>,
}

fn main() 
{
    let suits = vec!["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = vec!["Ace", "Two", "Three"];
    let mut cards = vec![];
  
    for suit in suits
    {
        for value in &values
        {
            let card = format!("{} of {}", &value, &suit);
            cards.push(card);
        }
    }
    
    let deck = Deck { cards: vec![ ]};

    println!("These are all the cards in the deck: {:?}", deck);
}
