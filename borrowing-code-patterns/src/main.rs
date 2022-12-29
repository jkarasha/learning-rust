// using mutable borrows

fn main() {
    let mut list = vec![1, 2, 3];

    {
        // example of immutable borrows with explicit scope definition.
        // We want to be explicit in our intent
        // These immutable borrows scope last only for the nested scope duration.
        let list_first = list.first();
        let list_last = list.last();


        // We are using both mutable and immutable borrows
        // Why doesn't this example throw E0502?
        // Because the mutation happens inline, no variable is left holding on to the value.
        
        println!(
            "The first element is {:?} and the last is {:?}",
            list_first,
            list_last,
        );
    }

    // Since we explicity defined the scope of the immutable borrows, this code should work
    *list.first_mut().expect("list was empty") += 1;

}
