fn main() {
    let list_a = vec![100,200,300,400];
    {
        let first_two = &list_a[0..2];
        println!("The first two elements are: {:?}", first_two)
    }
    // move after use/assign
    let list_b = list_a;
    println!("The first two elements are: {:?}", list_b)
}