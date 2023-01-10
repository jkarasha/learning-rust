fn main() {
    let first_two = {
        let list = vec![100, 200, 300, 400];
        &list[0..2]
    };

    println!("The first two elements are: {:?}", first_two)
}