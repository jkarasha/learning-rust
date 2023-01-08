fn main() {
    let list = vec![100,200, 300, 400, 500];
    println!("List contents: {:?}", list);
    //not the same first_two defined in print_first_two()
    let first_two = print_first_two_return(&list);
    println!("First two elements: {:?}", first_two);  
    //
    print_first_two(&list);
}

fn print_first_two(borrowed_list: &[i32]) {
    //first_two lifetime is limited to this function.
    let first_two = &borrowed_list[0..2];
    println!("First two elements: {:?}", first_two);
}

fn print_first_two_return(borrowed_list: &[i32]) -> &[i32] {
    &borrowed_list[0..2]
}