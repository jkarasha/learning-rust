//Examples of how to use Result & Option in error handling
//Result returns a success/failure. (Ok) or (Err)
//Option returns a sucess/nothing. (Some) or (None)

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Person  {
    name: String,
}

fn main() {
    let first = serde_json::from_str::<Person>(r#"{
        "name": "Margaret Kanyoko"
    }"#);
    println!("first = {:?}", first);
    // This generates invalid json
    let second = serde_json::from_str::<Person>(r#"{
        "name": "Margaret Kanyoko",
    }"#);
    println!("second = {:?}", second);
}
