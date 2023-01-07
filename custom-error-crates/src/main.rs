
//replace with error_chain
//extern crate quick_error;
#[macro_use]
extern crate error_chain;

use std::fs::{File, OpenOptions};

// removed
/*
use quick_error::ResultExt;
use std::{io, result};

type Result<T> = result::Result<T, DocumentServiceError>;
*/
// replace with error_chain
/*
quick_error! {
    #[derive(Debug)]
    pub enum DocumentServiceError {
        RateLimitExceeded {
            display("You have exceeded the allowed number of documents per minute!")
        }
        Io(filename: String, cause: io::Error) {
            display("I/O error: {} for filename {}", cause, filename)
            context(filename: &'a str, cause: io::Error)
                -> (filename.to_string(), cause)
        }
    }
}
*/
pub mod errors {
    error_chain! {
        errors {
            RateLimitExceeded {
                display("You have exceeded the allowable number of documents per minute.")
            }
        }
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

use errors::*;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        bail!(ErrorKind::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .chain_err(|| format!("could not open {}", filename))?;

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
        Err(e) => {
            println!("Project creation failed: {}", e);
            for e in e.iter().skip(1) {
                println!("Caused by: {}", e);
            }
            if let Some(backtrace) = e.backtrace() {
                println!("Backtrace: {:?}", backtrace);
            }
        }
    }
}