fn main() {
    let first_two = {
        let list = vec![100, 200, 300, 400];
        &list[0..2]
    };

    //attempting to use a value from an inner scope
    //borrowed value does not live long enough
    println!("The first two elements are: {:?}", first_two)
}