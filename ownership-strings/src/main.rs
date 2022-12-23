fn pluralize(s: String) -> String {
    //let mut new_value = s;
    //new_value.append_str("s")
    s + "s"
}

fn main() {
    let s = String::from("book");

    let pluralized = pluralize(s.clone());

    println!(
        "I have one {}, you have two {}",
        s,
        pluralized,
    );
}
