
// Show the iterator invalidation problem in rust.
fn main() {
    let mut list = vec![1, 2, 3];
    // although list is mutable, we won'be be able to modify it without making a copy/slice?
    // Not sure I understand fully at this time
    for i in &list {
        println!("i is {}", i);
        // here we try to add something to the list
        list.push(i + 1);
    }
}