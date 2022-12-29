// using mutable borrows

fn main() {
    let mut list = vec![1, 2, 3];

    // example of a mutable borrow
    // mutable borrow starts and ends on this line.
    // Notice the location of this code matters.
    // It happens before the immutable borrows.
    *list.first_mut().expect("list was empty") += 1;

    // example of immutable borrows.
    let list_first = list.first();
    let list_last = list.last();

    // IF we tried to line line 15 here instead, it would fail.
    // Because it's trying to mutable borrow of an already immutable borrowed object.
    //*list.first_mut().expect("list was empty") += 1;

    // We are using both mutable and immutable borrows
    // Why doesn't this example throw E0502?
    // Because the mutation happens inline, no variable is left holding on to the value.
    
    println!(
        "The first element is {:?} and the last is {:?}",
        list_first,
        list_last,
    );
}
