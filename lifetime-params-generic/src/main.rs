extern crate rand;

//adding 'a (tick-a) allows the code to compile.
fn simulate_game<'a>(home: &'a str, away: &'a str) -> &'a str {
    if rand::random() {
        home
    } else {
        away
    }
}

fn main() {
    println!("Hello, world!");
}
