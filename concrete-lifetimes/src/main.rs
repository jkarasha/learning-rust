fn main() {
    let list = vec![100,200, 300, 400, 500];
    println!("List contents: {:?}", list);
    //creates a reference to the first two elements of list
    let first_two = &list[0..2];
    println!("First two elements: {:?}", first_two);
    println!("List contents: {:?}", list);
    println!("Re-Print the first two elements: {:?}", first_two);
    //the lifetime of list and first_two last through end of main.
}
