// using temporary variable to resolve immutable + mutable borrow error.
pub struct Player {
    score: i32
}

// define a couple of object functions
impl Player {
    // Set score takes in a mutable self reference.
    pub fn set_score(&mut self, new_score: i32) {
        self.score = new_score;
    }
    // Get Score
    pub fn score(&self) -> i32 {
        self.score
    }
    // Initialize a zero score user.
    pub fn new() -> Self {
        Player { score: 0 }
    }
}

fn main() {
    let mut steph_curry = Player::new();
    // We attempt to increment the players score.
    // Notice this is both a mutable-borrow(set new score) and and immutable-borrow (get current score)
    // We'll use temp_variable in order to be more explicit about it.
    let current_score = steph_curry.score();    
    steph_curry.set_score(current_score + 3);
    let current_score = steph_curry.score();    
    steph_curry.set_score(current_score + 2);
    //
    println!("Points: {}", steph_curry.score());
}
