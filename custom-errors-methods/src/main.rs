extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
}

fn main() {
    let first = serde_json::from_str::<Person>(r#"{
        "name": "Margret Kanyoko",
    }"#);

    //unwrap of use provided default.
    let first_inner = first.unwrap_or(Person { name: String::from("unknown")});

    println!("First's name = {:?}", first_inner.name);
}
