//using custom errors
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::fmt;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

//We use the derive macro in order to inclue enum details in the debug information.
#[derive(Debug)]
pub enum DocumentServiceError {
    RateLimitExceeded,
    Io(io::Error),    
}

//custom error handlers must implement the error trait
//easily done by adding a definition for the description function as show below.
//rust versions newer that 1.27 don't have to implement the method body
//impl Error for DocumentServiceError {} is good enough
impl Error for DocumentServiceError {
    fn description(&self) -> &str {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => "rate limit exceeded",
            Io(_) => "I/O error",
        }
    }
}

//implementing Error trait also requires we implement debug/display traits
impl fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => write!(
                f,
                "You have exceeded the allow number of documents per minute"
            ),
            Io(ref io) => write!(
                f,
                "I/O error: {}", io
            ),
        }
    }
}

//implementing the From trait allows us to convert from one type to another
//in this case we can convert from io:Error to DocumentService:Io Error.
impl From<io::Error> for DocumentServiceError {
    // Self here indicates we are returning a DocumentServiceError type
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other)
    }
}

use std::result;

//create an alias for the Result<T> that returns a DocumentServiceError
//will greatly simply the code in our "crate" since most functions return this type.
pub type Result<T> = result::Result<T, DocumentServiceError>;

//Why is this declared at public? Works either way
pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        // because we implemented the from trait, ? now handles the type conversion for us
        .open(filename)?;

    Ok(file)
}

// Notice the DocumentServiceError type now isn't explicitly called
//that's because we defined an alias for our result type
pub fn archive_document(filenam: &str) -> Result<()> {
    unimplemented!();
}

pub fn list_documents() -> Result<Vec<File>> {
    unimplemented!();
}
fn main () {
    println!("{:?}", create_document("docs.txt"));
}