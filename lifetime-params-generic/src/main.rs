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

    let winner = {
        let team_two = String::from("Bills");
        
        //error[E0597]: `team2` does not live long enough
        simulate_game(&team_one, &team_two)
    };

    //now both team_one and winner are available here.
    //notice team_two is only available in the nested scope.
    println!("I can't believe {:#?} won", winner);
}
