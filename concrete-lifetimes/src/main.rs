//Struct shouldn't refer to themselves.
//error[E0106]: missing lifetime specifier
struct ListAndRef {
    list: Vec<i32>,
    first_two: &[i32],
}

fn return_list_and_first_two() -> ListAndRef {
    let list_to_use= vec![100,200,300,400];

    ListAndRef {
        list: list_to_use,
        first_two: &list_to_use,
    }
}
fn main() {
    println!("This shouldn't work!")
}