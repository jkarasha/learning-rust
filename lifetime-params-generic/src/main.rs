extern crate rand;

//will throw error[E0106]: missing lifetime specifier
//help: this function's return type contains a borrowed value, 
//but the signature does not say whether it is borrowed from `home` or `away`
fn simulate_game(home: &str, away: &str) -> &str {
    if rand::random() {
        home
    } else {
        away
    }
}

fn main() {
    println!("Hello, world!");
}
