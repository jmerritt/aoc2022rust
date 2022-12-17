use std::fs;
use std::collections::HashMap;

fn main() {

    let file = fs::read_to_string("input.txt")
        .expect("couldn't get the goods");

    let mut outcomes = HashMap::new();
    outcomes.insert(String::from("A X"), 4); // rock vs rock is tie = 1 + 3
    outcomes.insert(String::from("A Y"), 8); // rock vs paper is win = 2 + 6
    outcomes.insert(String::from("A Z"), 3); // rock vs scissors is loss = 3 + 0
    outcomes.insert(String::from("B X"), 1); // paper vs rock is loss = 1 + 0
    outcomes.insert(String::from("B Y"), 5); // paper vs paper is tie = 2 + 3
    outcomes.insert(String::from("B Z"), 9); // paper vs scissors is win = 3 + 6
    outcomes.insert(String::from("C X"), 7); // scissors vs rock is win = 1 + 6
    outcomes.insert(String::from("C Y"), 2); // scissors vs papar is loss = 2 + 0
    outcomes.insert(String::from("C Z"), 6); // scissors vs scissors is tie = 3 + 3

    let mut score1 = 0;
    for game in file.lines() {
        match outcomes.get(game) {
            Some(points) => score1 += points,
            None => break,
        }
    }
    // result for part 1
    println!("final score: {}", score1);

    let mut decisions = HashMap::new();
    decisions.insert(String::from("A X"), 3); // lose to rock = 3 + 0
    decisions.insert(String::from("A Y"), 4); // tie with rock = 1 + 3
    decisions.insert(String::from("A Z"), 8); // win to rock = 2 + 6
    decisions.insert(String::from("B X"), 1); // lose to paper = 1 + 0
    decisions.insert(String::from("B Y"), 5); // tie with paper = 2 + 3
    decisions.insert(String::from("B Z"), 9); // win to paper = 3 + 6
    decisions.insert(String::from("C X"), 2); // lose to scissors = 2 + 0
    decisions.insert(String::from("C Y"), 6); // tie with scissors = 3 + 3
    decisions.insert(String::from("C Z"), 7); // win to scissors = 1 + 6

    let mut score2 = 0;
    for game in file.lines() {
        match decisions.get(game) {
            Some(points) => score2 += points,
            None => break,
        }
    }
    // result for part 2
    println!("final score2: {}", score2);
}
