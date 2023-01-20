fn main() {
    let first_two = return_first_two();

    println!("First two elements are {:?}", first_two);
}

//won't compile: error[E0106]: missing lifetime specifier
//&[i32] return type expected named lifetime parameter
//help: this function's return type contains a borrowed value, 
//but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
fn return_first_two() -> &[i32] {
    let list = vec![100,200,300,400];
    &list[0..2]
}