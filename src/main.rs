
use std::fs;
use std::io::Error;

use regex::Regex;

/*
 * Global Constants
 */

const DATE_PATTERN : &str = r"(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})Z"; //-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z";

// Anchor days for doomsday algorithm
const ANCHOR_DAY_1900 : DayOfWeek = DayOfWeek::WEDNESDAY;
const ANCHOR_DAY_2000 : DayOfWeek = DayOfWeek::TUESDAY;
const ANCHOR_DAY_2100 : DayOfWeek = DayOfWeek::SUNDAY;

/*
 *
 */

/*
 * Day Of Week enum
 * and associated methods
 */

enum DayOfWeek {
  MONDAY,
  TUESDAY,
  WEDNESDAY,
  THURSDAY,
  FRIDAY,
  SATURDAY,
  SUNDAY,
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

/*
 *
 */

/*
 * Month of Year enum
 * with associated methods
 */

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
  DECEMBER,
}

impl MonthOfYear {
  fn as_month_number(&self) -> isize {
    match self {
      MonthOfYear::JANUARY => 1,
      MonthOfYear::FEBRUARY => 2,
      MonthOfYear::MARCH => 3,
      MonthOfYear::APRIL => 4,
      MonthOfYear::MAY => 5,
      MonthOfYear::JUNE => 6,
      MonthOfYear::JULY => 7,
      MonthOfYear::AUGUST => 8,
      MonthOfYear::SEPTEMBER => 9,
      MonthOfYear::OCTOBER => 10,
      MonthOfYear::NOVEMBER => 11,
      MonthOfYear::DECEMBER => 12,
    }
  }
  fn from_month_number(month_number : isize) -> Option<MonthOfYear> {
    match month_number {
      1 => Some(MonthOfYear::JANUARY),
      2 => Some(MonthOfYear::FEBRUARY),
      3 => Some(MonthOfYear::MARCH),
      4 => Some(MonthOfYear::APRIL),
      5 => Some(MonthOfYear::MAY),
      6 => Some(MonthOfYear::JUNE),
      7 => Some(MonthOfYear::JULY),
      8 => Some(MonthOfYear::AUGUST),
      9 => Some(MonthOfYear::SEPTEMBER),
      10 => Some(MonthOfYear::OCTOBER),
      11 => Some(MonthOfYear::NOVEMBER),
      12 => Some(MonthOfYear::DECEMBER),
      _ => None
    }
  }
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
      MonthOfYear::DECEMBER => 12,
    }
  }
}

/*
 *
 */

/*
 * DateTime struct
 * with associated methods
 */

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
    // Determine whether this is a leap year
    let leap_year = self.is_leap_year();
    // Get anchor date number for current month
    let this_month = MonthOfYear::from_month_number(self.month);
    let month_doomsday;
    match this_month {
      Some(m) => {
        month_doomsday = m.anchor_date(leap_year);
      },
      None => {
        return Err(
          format!(
            "Month number {} out of range",
            self.month
          )
        );
      }
    }
    // Calculate day number from doomsday date & day number
    let mut date_counter = month_doomsday;
    let mut day_counter = doomsday_number;
    while date_counter != self.day {
      if date_counter > self.day {
        date_counter -= 1;
        day_counter -= 1;
      } else {
        date_counter += 1;
        day_counter += 1;
      }
      if day_counter < 0 {
        day_counter = 6;
      }
      if day_counter > 6 {
        day_counter = 0;
      }
    }
    // Return day of week, if possible
    match DayOfWeek::from_day_number(day_counter) {
      Some(day_of_week) => {
        Ok(day_of_week)
      }
      None => {
        Err(
          format!(
            "Doomsday algorithm found day number {}, out of range",
            day_counter
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

/*
 *
 */

/*
 * Functions used in main
 */

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

/*
 *
 */

/*
 * Time bin struct and associated methods
 */
struct TimeBin {
  lower_limit : isize,  // Minutes from 00:00
  upper_limit : isize,  // Minutes from 00:00
  count : isize,
}

impl TimeBin {
  // Return a range of time bins
  fn range(start_time : isize, interval : isize, number : isize)
    -> Result<Vec<TimeBin>, String> {
    // Check all bins falls within a single day
    if (start_time + number*interval) > 60*24 {
      Err("Requested range spans more than a whole day".into_string())
    } else {
      let mut bins : Vec<TimeBin> = vec!();
      for i in 0..number {
        bins.push(
          TimeBin {
            lower_limit: start_time + interval*i,
            upper_limit: start_time + interval*(i+1),
            count: 0
          }
        );
      }
      Ok(bins)
    }
  }
  fn add(&mut self, datetime : DateTime) -> bool {
    let time_to_check = datetime.hour*60 + datetime.minute;
    if (time_to_check >= self.lower_limit) && (time_to_check < self.upper_limit) {
      self.count += 1;
      true
    } else {
      false
    }
  }
}

fn main() {

  let filename : &str = "/home/scott/Documents/Projects/BestTimeGraph/test-data.txt";

  let read_file : Result<String, Error> = fs::read_to_string(filename);

  // Collect results here
  let mut data_points : Vec<DateTime> = vec!();

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
                    data_points.push(d);
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

  // Sort data into days
  let mut days : Vec<Vec<DateTime>> = vec!(
    vec!(),
    vec!(),
    vec!(),
    vec!(),
    vec!(),
    vec!(),
    vec!()
  );

  for data_point in data_points {
    match data_point.weekday() {
      Ok(dow) => {
        match days.get_mut(dow.as_day_number() as usize) {
          Some(d) => {
            d.push(data_point);
          }
          None => {
            println!(
              "Could not push {}, found invalid day number {}",
              data_point,
              dow.as_day_number()
            );
          }
        }
      }
      Err(e) => {
        println!(
          "Sorting data in days failed for {} with message {}",
          data_point,
          e
        );
      }
    }
  }

  // Check
  for i in 0..days.len() {
    println!(
      "On {}th day, {} data points",
      i,
      days[i].len()
    );
  }

}