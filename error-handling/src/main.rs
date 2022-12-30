//show how to use custom panics from  external crates 
//make sure to include dependency on http in cargo.toml file.
extern crate http;

use http::header::HeaderName;

fn main() {
    // http headers cannot include special characters.
    //thread 'main' panicked at 'invalid header name', /home/jokarash/.cargo/registry/src/github.com-1ecc6299db9ec823/http-0.1.10/src/header/name.rs:1656:23
    //let _h = HeaderName::from_static("special! characters!");
    let _h = HeaderName::from_static("no-special-characters");

}