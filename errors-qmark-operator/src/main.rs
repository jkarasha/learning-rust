
fn parse_number(n: &str) -> i32 {
    //parse returns a Result<> type
    // expected `i32`, found enum `Result`
    //let num: i32 = n.parse();
    // if we try to use ? it will also fail.
    // cannot use the `?` operator in a function that returns `i32`
    let num: i32 = n.parse()?;
    num
}

fn main() {
    println!("Number: ", parse_number("three"));
}