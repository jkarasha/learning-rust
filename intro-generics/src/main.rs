extern crate rand;

//this will generate error[E0106]: missing lifetime specifier error
// rust has no way to guaranteeing returned value [home|away] will be around long enough
fn simulate_game(home: &str, away: &str) -> &str {
    //randomly return winner
    if rand::random() {
        home
    } else  {
        away
    }
}

fn main() {
    let team_one = String::from("Patriots");
    //because team two is defined in nested scope,
    //we have no guarantee value will be around by the time we print
    //this causes this code to also fail. Same error error[E0106]: missing lifetime specifier
    let winner = {
        let team_two = String::from("Bills");
        simulate_game(&team_one, &team_two)
    };
    //println!("Final match between {} and {}: {} won!", team_one, team_two, winner);
    println!("I think {} should have won!", winner);
}
