
use std::fs;
use std::io::Error;

fn main() {

  let filename : &str = "/home/scott/Documents/Projects/BestTimeGraph/test-data.txt";

  let read_file : Result<String, Error> = fs::read_to_string(filename);

  match read_file {
    Ok(f) => println!("{}", f),
    Err(description) => println!("Error {}", description)
  }

}