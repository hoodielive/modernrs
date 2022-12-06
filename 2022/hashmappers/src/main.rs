use std::collections::HashMap;

fn main()
{
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
   // scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Yellow")).or_insert(25);

    println!("This is the HashMap {:?}", scores.keys()); 
    println!("This is the HashMap {:?}", scores.values()); 

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The team is {} and the score is {}.", team_name, score);

    for (key, value) in &scores
    {
        println!("{}: {}", key, value)
    };

    let field_name = String::from("Favorite Color");
    let field_value = String:: from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{:?} is in the map.", map);
    println!("{:?} is in the map.", &field_name);

}
