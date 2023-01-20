fn main() {
    let first_two = return_first_two();

    println!("First two elements are {:?}", first_two);
}

static LIST: [i32; 4] = [100,200,300,400];

//switch to use a static list instead
fn return_first_two() -> &'static [i32] {
    &LIST[0..2]
}