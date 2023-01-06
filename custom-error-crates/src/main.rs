//use the DocumentServiceError as starting point
//replace functionality with error crates where relevant
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::{fmt, io, result};

type Result<T> = result::Result<T, DocumentServiceError>;

#[derive(Debug)]
pub enum DocumentServiceError {
    RateLimitExceeded,
    Io(io::Error),
}

//implement Error trait
impl Error for DocumentServiceError {
    fn description(&self) -> &str {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => "Rate limit exceeded",
            Io(_) => "I/O Error",
        }
    }
}

//implement Format trait
impl fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => write!(
                f,
                "You have exceeded the allowed number of documents per minute",
            ),
            Io(ref io) => write!(f, "I/O Error: {}", io),
        }
    }
}

//implement From trait
impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other)
    }
}

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

fn create_project(project_name: &str) -> Result<()> {
    create_document(&format!("{}-draft-1", project_name))?;
    create_document(&format!("{}-draft-2", project_name))?;
    create_document(&format!("{}-revision-1", project_name))?;
    create_document(&format!("{}-revision-2", project_name))?;
    Ok(())
}

fn main() {
    match create_project("my-project") {
        Ok(()) => println!("Project created successfully!"),
        Err(e) => println!("Project creation failed: {}", e),
    }
}