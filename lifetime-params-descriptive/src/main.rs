fn main() {
    let first_two = return_first_two();

    println!("First two elements are {:?}", first_two);
}

//instead 'a trying adding 'static (static-lifetime)
//error[E0515]: cannot return value referencing local variable `list`
fn return_first_two() -> &'static [i32] {
    let list = vec![100,200,300,400];
    &list[0..2]
}