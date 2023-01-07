fn main() {
    //wraps 25 inside of the option value
    let option_value = Some(25);
    if option_value.is_some() {
        //unwrap the option value and extract 25
        let inner = option_value.unwrap();
        println!("Inner: {}", inner);
    }
}
