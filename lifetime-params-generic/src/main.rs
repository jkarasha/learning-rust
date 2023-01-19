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
    let team_one = String::from("Patriots");
    {
        let team_two = String::from("Bills");
        let winner = simulate_game(&team_one, &team_two);
        println!("{} vs. {}: {} won", team_one, team_two, winner);
    }
    //if you tried to print the winner here, code will fail
    //winner's lifetime is only within nested scope.
    println!("I can't believe {} won", winner);
}
