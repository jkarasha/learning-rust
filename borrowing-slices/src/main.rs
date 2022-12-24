// slicing works very similiar to how it works in Python
// [begin..end], starts at "begin" and stops before "end"
// begin is inclusive, end is not

fn main() {
    let greeting = String::from("Hello, World!");
    //
    let slice = &greeting[2..4];
    println!("The slice from 2 to 4 is {}.", slice);
    // returns the first 4 chars   
    let slice = &greeting[0..4];
    println!("The first four chars in greeting are {}.", slice);
    let slice = &greeting[..4];
    println!("The slice without a starting index up to 4 is {}", slice);
    let slice = &greeting[2..];
    println!("The slice from 2 without an ending index is {}", slice);
    let slice = &greeting[..];
    println!("The slice of the entire string is {}", slice);

}
