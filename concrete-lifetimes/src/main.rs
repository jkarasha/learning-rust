fn main() {
    //creates list in main function
    let list = return_first_two();
    let first_two = &list[0..2];
    //lifetime is now valid
    println!("The first two elements are: {:?}", first_two)
}

fn return_first_two() -> Vec<i32> {
    vec![100, 200, 300, 400]
}