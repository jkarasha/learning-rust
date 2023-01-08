fn main() {
    let list = vec![100,200, 300, 400, 500];
    // use inner scope to change first-two lifetime
    {
        println!("List contents: {:?}", list);
        let first_two = &list[0..2];
        println!("First two elements: {:?}", first_two);
    }
    //
    println!("List contents: {:?}", list);
    //attempting to print first_two will throw error
    //error[E0425]: cannot find value `first_two` in this scope
    //println!("Re-Print the first two elements: {:?}", first_two);
    //the lifetime of list lasts through end of main.
}
