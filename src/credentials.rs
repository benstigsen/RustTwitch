use std::fs::File;
use std::io::{prelude::*, BufReader, Error};
use std::env;

pub fn get_credentials(filename: &str) -> Result<Vec<String>, Error> {
	//println!("PATH: {}", env::current_exe()?.display());

	let file = File::open(filename).unwrap();
	let buf = BufReader::new(file);

	let mut contents = Vec::new();

	for line in buf.lines() {
		contents.push(line.unwrap())

	}
	
	Ok(contents)
}
