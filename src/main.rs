
use std::fs;
use std::io::Error;

use regex::Regex;

const DATE_PATTERN : &str = r"(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})Z"; //-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z";

struct DateTime {
  year: isize,
  month: isize,
  day: isize,
  hour: isize,
  minute: isize,
  second: isize
}

impl std::fmt::Display for DateTime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}-{}-{} {}:{}:{}",
      self.year,
      self.month,
      self.day,
      self.hour,
      self.minute,
      self.second
    )
  }
}

fn parse_date(date_string : &str) -> Option<DateTime> {

  let date_regex : Regex = Regex::new(DATE_PATTERN).unwrap();

  let parts = date_regex.captures(date_string);

  let year : isize;
  let month : isize;
  let day : isize;
  let hour : isize;
  let minute : isize;
  let second : isize;

  match parts {

    Some(captures) => {

      match captures.get(1) {

        None => {
          return Option::None;
        }

        Some(year_string) => {
          match year_string.as_str().parse::<isize>() {
            Ok(year_int) => {
              year = year_int;
            }
            Err(e) => {
              return Option::None;
            }
          }
        }

      }

      match captures.get(2) {

        None => {
          return Option::None;
        }

        Some(month_string) => {
          match month_string.as_str().parse::<isize>() {
            Ok(month_int) => {
              month = month_int;
            }
            Err(e) => {
              return Option::None;
            }
          }
        }

      }

      match captures.get(3) {

        None => {
          return Option::None;
        }

        Some(day_string) => {
          match day_string.as_str().parse::<isize>() {
            Ok(day_int) => {
              day = day_int;
            }
            Err(e) => {
              return Option::None;
            }
          }
        }

      }

      match captures.get(4) {

        None => {
          return Option::None;
        }

        Some(hour_string) => {
          match hour_string.as_str().parse::<isize>() {
            Ok(hour_int) => {
              hour = hour_int;
            }
            Err(e) => {
              return Option::None;
            }
          }
        }

      }


      match captures.get(5) {

        None => {
          return Option::None
        }

        Some(minute_string) => {
          match minute_string.as_str().parse::<isize>() {
            Ok(minute_int) => {
              minute = minute_int;
            }
            Err(e) => {
              return Option::None;
            }
          }
        }

      }

      match captures.get(6) {

        None => {
          return Option::None;
        }

        Some(second_string) => {
          match second_string.as_str().parse::<isize>() {
            Ok(second_int) => {
              second = second_int;
            }
            Err(e) => {
              return Option::None;
            }
          }
        }

      }

      return Some(
        DateTime {
          year: year,
          month: month,
          day: day,
          hour: hour,
          minute: minute,
          second: second
        }
      )
    }

    None => {
      return Option::None;
    }

  }
}

fn main() {

  let filename : &str = "/home/scott/Documents/Projects/BestTimeGraph/test-data.txt";

  let read_file : Result<String, Error> = fs::read_to_string(filename);

  match read_file {
    Ok(contents) => {
      for datetime_string in contents.lines() {
        let comma_position = datetime_string.find(",");
        match comma_position {
          Some(i) => {
            match parse_date(&datetime_string[0..i]) {
              Some(d) => {
                println!("From {} parsed {}", datetime_string, d);
              }
              None => {
                println!("Could not parse {}, {}", datetime_string, &datetime_string[0..i]);
              }
            }
          }
          None => {
            println!("Could not find a comma on this line: {}", datetime_string);
          }
        }

      }
    }
    Err(description) => {
      println!("Could not read file, error {}", description);
    }
  }

}