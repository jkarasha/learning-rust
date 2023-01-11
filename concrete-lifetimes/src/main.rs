fn main() {
    //tries to retun a reference from a function.
    //error[E0106]: missing lifetime specifier
    let first_two = return_first_two();
    println!("The first two elements are: {:?}", first_two)
}

fn return_first_two() -> &[i32] {
    //because list is defined in the function, a slice of list won't be available outside of function.
    let list = vec![100, 200, 300, 400];
    &list[0..2]
}