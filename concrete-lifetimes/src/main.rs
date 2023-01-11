fn main() {
    //attempts to use a moved value
    let list_a = vec![100,200,300,400];
    let list_b = list_a;
    //try to access value from moved list_a
    //error[E0382]: borrow of moved value: `list_a`
    let first_two = &list_a[0..2];
    println!("The first two elements are: {:?}", first_two)
}