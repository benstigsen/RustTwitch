use std::fs::File;
use std::io::{prelude::*, BufReader, Error};

// Load credentials
pub fn get_credentials(filename: &str) -> Result<Vec<String>, Error> {

    let file = File::open(filename).unwrap(); // Open file
    let buf = BufReader::new(file);           // Read file into buffer

    let mut contents = Vec::new(); // Vector for file content

    for line in buf.lines() {
        contents.push(line.unwrap()) // Save line in file to vector
    }
    
    Ok(contents)
}
