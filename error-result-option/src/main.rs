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
        "name": "Margaret Kanyoko"
    }"#);
    // returns an Result object with a valid (Ok) response.
    println!("Person is: {:?}", first);
    //However trying to read it's name attribute results in an error, type mismatch
    //error[E0609]: no field `name` on type `Result<Person, serde_json::Error>`
    println!("Person's first name is: {:?}", first.name);
}