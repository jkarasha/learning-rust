// using mutable borrows

fn main() {
    let list = vec![1, 2, 3];

    // example of immutable borrows.
    let list_first = list.first();
    let list_last = list.last();

    // Now, let's add a mutable borrow
    // first_mut() returns a mutable pointer to the first element of the slice, or None if it is empty.
    // This code will throw the error[E0596]: cannot borrow `list` as mutable, as it is not declared as mutable
    // We are trying to borrow an both both as immutable and now mutable.
    let list_first_mut = list.first_mut().expect("list was empty");
    println!("The first element of the list is {}", list_first_mut);
    *list_first_mut += 1;

    println!(
        "The first element is {:?} and the last is {:?}",
        list_first,
        list_last,
    );
}
