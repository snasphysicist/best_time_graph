
use std::fs;
use std::io::Error;

use regex::Regex;

const DATE_PATTERN : &str = r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z)";

struct DateTime {
  year: isize,
  month: isize,
  day: isize,
  hour: isize,
  minute: isize,
  second: isize
}

fn parse_date(date_string : &str) -> (isize, isize, isize, isize, isize, isize) {
  let
}

fn main() {

  let filename : &str = "/home/scott/Documents/Projects/BestTimeGraph/test-data.txt";

  let read_file : Result<String, Error> = fs::read_to_string(filename);

  match read_file {
    Ok(f) => println!("{}", f),
    Err(description) => println!("Error {}", description)
  }

}