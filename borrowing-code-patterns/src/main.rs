// using Map's Entry-API to resolve immutable + mutable borrow error.
use std::collections::HashMap;

fn main() {
    
    let text = String::from("Hello world, Hello again!");
    // We'll use Map to count the word frequency.
    let mut freqs = HashMap::new();
    for word in text.split_whitespace() {
        //This is much explicit
        *freqs.entry(word).or_insert(0) += 1;
        // instead of trying to use a match
        /* match freqs.get_mut(word) {
            Some(value) => *value += 1,
            None => {
                // We t
                freqs.insert(word, 1);
            },
        } */
    }

    println!("Word frequencies: {:#?}", freqs);
}
