use std::io;
use std::collections::HashMap;

fn main() {
    println!("Please enter some words to get a wordcount!");
    
    let mut input = String::new();
    let mut counts = HashMap::new();
    
    io::stdin().read_line(&mut input).expect("Problem reading input!");

    for word in input.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Counts = {:?}", counts);
}