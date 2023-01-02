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
        "name": "Margaret Kanyoko",
    }"#);
    // returns an Result object with a valid (Ok) response.
    println!("Person is: {:?}", first);
    //However trying to read it's name attribute results in an error, type mismatch
    //error[E0609]: no field `name` on type `Result<Person, serde_json::Error>`
    // println!("Person's first name is: {:?}", first.name);

    //inner is a user defined variable, could be called anything.
    //in the event of an error, you can return custom types for more details.
    let first_inner = match first {
        Ok(inner) => inner,
        Err(_) => Person {
            name: String::from("unknown"),
        },
        _ => unimplemented!(),
    };

    println!("First person is: {:?}", first_inner);
    println!("First person's name is: {:?}", first_inner.name);
}