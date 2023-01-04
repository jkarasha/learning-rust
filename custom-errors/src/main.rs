//many times you have a function that can return different types of errors.
//What will the error return type be??
fn num_threads() -> Result<usize, ???> {
    //below line will return a VarErr??
    let s = env::var("NUM_THREADS")?;
    //below line will return IntParseError
    let n: usize = s.parse()?;
    Ok(n + 1)
}

fn main() {
}
