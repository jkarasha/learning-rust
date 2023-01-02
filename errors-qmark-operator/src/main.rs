
// instead of return i32, update to return Result<>
// i32 on success and ParseIntError on failure
fn parse_number(n: &str) -> Result<i32, std::num::ParseIntError> {
    let num: i32 = n.parse()?;
    Ok(num)
}

fn main() {
    println!("Number: {:?}", parse_number("3"));
}