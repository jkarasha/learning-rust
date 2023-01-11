fn main() {
    //one way to fix previous issue, by moving everything in the same scope
    let list = vec![100, 200, 300, 400];
    let first_two = &list[0..2];
    println!("The first two elements are: {:?}", first_two)
}