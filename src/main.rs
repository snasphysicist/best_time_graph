
use std::fs;
use std::io::Error;

use regex::Regex;

const DATE_PATTERN : &str = r"(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})Z"; //-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z";

// Anchor days for doomsday algorithm
const ANCHOR_DAY_1900 : DayOfWeek = DayOfWeek::WEDNESDAY;
const ANCHOR_DAY_2000 : DayOfWeek = DayOfWeek::TUESDAY;
const ANCHOR_DAY_2100 : DayOfWeek = DayOfWeek::SUNDAY;

enum DayOfWeek {
  MONDAY,
  TUESDAY,
  WEDNESDAY,
  THURSDAY,
  FRIDAY,
  SATURDAY,
  SUNDAY,
}

enum MonthOfYear {
  JANUARY,
  FEBRUARY,
  MARCH,
  APRIL,
  MAY,
  JUNE,
  JULY,
  AUGUST,
  SEPTEMBER,
  OCTOBER,
  NOVEMBER,
  DECEMBER
}

// TODO DateTime anchor date, calls MonthOfYear anchor date
// TODO Week day from anchor date/day

impl MonthOfYear {
  fn anchor_date(&self, is_leap_year : bool) -> isize {
    match self {
      MonthOfYear::JANUARY => {
        if is_leap_year {
          4
        } else {
          3
        }
      },
      MonthOfYear::FEBRUARY => {
        if is_leap_year {
          29
        } else {
          28
        }
      },
      MonthOfYear::MARCH => 7,
      MonthOfYear::APRIL => 4,
      MonthOfYear::MAY => 9,
      MonthOfYear::JUNE => 6,
      MonthOfYear::JULY => 11,
      MonthOfYear::AUGUST => 8,
      MonthOfYear::SEPTEMBER => 5,
      MonthOfYear::OCTOBER => 10,
      MonthOfYear::NOVEMBER => 7,
      MonthOfYear::DECEMBER => 12
    }
  }
}

impl DayOfWeek {
  fn as_day_number(&self) -> isize {
    match self {
      DayOfWeek::SUNDAY => 0,
      DayOfWeek::MONDAY => 1,
      DayOfWeek::TUESDAY => 2,
      DayOfWeek::WEDNESDAY => 3,
      DayOfWeek::THURSDAY => 4,
      DayOfWeek::FRIDAY => 5,
      DayOfWeek::SATURDAY => 6,
    }
  }
  fn from_day_number(day_number : isize) -> Option<DayOfWeek> {
    match day_number {
      0 => Some(DayOfWeek::SUNDAY),
      1 => Some(DayOfWeek::MONDAY),
      2 => Some(DayOfWeek::TUESDAY),
      3 => Some(DayOfWeek::WEDNESDAY),
      4 => Some(DayOfWeek::THURSDAY),
      5 => Some(DayOfWeek::FRIDAY),
      6 => Some(DayOfWeek::SATURDAY),
      _ => None,
    }
  }
}

impl std::fmt::Display for DayOfWeek {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let day_name = {
      match &self {
        DayOfWeek::SUNDAY => "Sunday",
        DayOfWeek::MONDAY => "Monday",
        DayOfWeek::TUESDAY => "Tuesday",
        DayOfWeek::WEDNESDAY => "Wednesday",
        DayOfWeek::THURSDAY => "Thursday",
        DayOfWeek::FRIDAY => "Friday",
        DayOfWeek::SATURDAY => "Saturday",
      }
    };
    write!(f, "{}", day_name)
  }
}

struct DateTime {
  year: isize,
  month: isize,
  day: isize,
  hour: isize,
  minute: isize,
  second: isize
}

impl DateTime {
  fn is_leap_year(&self) -> bool {
    self.year % 400 == 0        // Year divisible by 400
    || (
      self.year % 4 == 0        // Or year divisible by 4
      && self.year % 100 == 0   // But not 100
    )
  }
  // Doomsday algorithm
  fn weekday(&self) -> Result<DayOfWeek,String> {
    // Last two digits of year (ylt)
    let year_last_two = self.year % 100;
    // Quotient when divided by 12 (nt)
    let number_of_12s = year_last_two / 12;
    // Last two modulo 12
    let last_two_mod_12 = year_last_two % 6;
    // Quotient when mod 12 divided by 4
    let number_of_4s = last_two_mod_12 / 4;
    // Anchor day for this century
    let anchor_day = {
      match self.year - year_last_two {
        1900 => ANCHOR_DAY_1900.as_day_number(),
        2000 => ANCHOR_DAY_2000.as_day_number(),
        2100 => ANCHOR_DAY_2100.as_day_number(),
        // Can't handle dates < 1900, > 2199
        _ => return Err(
          format!(
            "Year {} outside of handled centuries",
            self.year
          )
        )
      }
    };
    // Sum of results mod 7 is day as int
    let doomsday_number = (
      number_of_12s + last_two_mod_12
        + number_of_4s + anchor_day
    ) % 7;
    let leap_year = self.is_leap_year();




    // Return day of week, if possible
    match DayOfWeek::from_day_number(day_number) {
      Some(day_of_week) => {
        Ok(day_of_week)
      }
      None => {
        Err(
          format!(
            "Doomsday algorithm found day number {}, out of range",
            day_number
          )
        )
      }
    }
  }
}

impl std::fmt::Display for DateTime {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    // Could read file
    Ok(contents) => {
      for datetime_string in contents.lines() {
        // Try to split into date , data
        let comma_position = datetime_string.find(",");
        match comma_position {
          // Could split by first comma
          Some(i) => {
            // Try to read the date into a DateTime
            match parse_date(&datetime_string[0..i]) {
              Some(d) => {
                println!("From {} parsed {}", datetime_string, d);
                // Try to get day of week
                match d.weekday() {
                  Ok(dow) => {
                    println!("Day is a {}", dow);
                  }
                  Err(e) => {
                    println!("Encountered error when converting to weekday: {}", e);
                  }
                }
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