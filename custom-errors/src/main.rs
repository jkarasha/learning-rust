//using custom errors
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

pub enum DocumentServiceError {
    RateLimitExceeded,
    Io(io::Error),    
}
//Why is this declared at public? Works either way
pub fn create_document(filename: &str) -> Result<File, Box<dyn Error>> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err("You have exceeded allowed docs per minute!".into());
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

fn main () {
    println!("{:?}", create_document("docs.txt"));
}