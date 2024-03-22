

use std::collections::HashMap;
fn main(){
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 20);

    println!("The score is : {:?}", scores);

    // Get a value by key

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The score of the {} is {}", team_name,score);

    // Loop hashmap

    for (key, value) in &scores {
        println!("--- {key}: {value} ---");
    }

    // Adding a value only if not exist with ENTRY

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("The update score is : {:?}", scores);

    // Count words in a string or &str

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!(" Map = {:?}", map);





}