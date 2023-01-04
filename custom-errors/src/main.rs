//many times you have a function that can return different types of errors.
use std::error::Error;
use std::env;
//We'll use Box<Error> to handle multiple errors in same function..
fn num_threads() -> Result<usize, Box<dyn Error>> {
    //below line will return a VarErr??
    let s = env::var("NUM_THREADS")?;
    //below line will return IntParseError
    let n: usize = s.parse()?;
    Ok(n + 1)
}

fn main() {
    println!("{:?}", num_threads());
}
