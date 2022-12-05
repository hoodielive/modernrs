use std::collections::HashMap;

fn main()
{
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("This is the HashMap {:?}", scores.keys()); 
    println!("This is the HashMap {:?}", scores.values()); 

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The team is {} and the score is {}.", team_name, score);
}
