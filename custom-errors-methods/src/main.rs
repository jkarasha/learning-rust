extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            name: String::from("unknown"),
        }
    }
}

fn main() {
    let first = serde_json::from_str::<Person>(r#"{
        "name": "Margret Kanyoko",
    }"#);

    //default type is provided by the struct
    let first_inner = first.unwrap_or_default();

    println!("First's name = {:?}", first_inner.name);
}
