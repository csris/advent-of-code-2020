#[macro_use]
extern crate clap;

use std::collections::HashSet;
use std::error::Error;
use std::io::prelude::*;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct ParseError;

#[derive(Debug)]
enum HeightUnits {
    Centimeters,
    Inches,
}

#[derive(Debug)]
struct Height {
    value: u32,
    units: HeightUnits,
}

impl Height {
    fn new(value: u32, units: HeightUnits) -> Height {
        if !Height::is_valid_value(value, &units) {
            panic!("Error: height {} {:?} out of range", value, units);
        }

        Height { value, units }
    }

    fn is_valid_value(value: u32, units: &HeightUnits) -> bool {
        match units {
            HeightUnits::Centimeters => 150 <= value && value <= 193,
            HeightUnits::Inches => 59 <= value && value <= 76,
        }
    }
}

impl FromStr for Height {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hgt, units) = s.split_at(s.len() - 2);

        let units = match units {
            "cm" => HeightUnits::Centimeters,
            "in" => HeightUnits::Inches,
            _ => return Err(ParseError{}),
        };

        let hgt = hgt.parse::<u32>().map_err(|_| ParseError{})?;

        if !Height::is_valid_value(hgt, &units) {
            return Err(ParseError{});
        }

        Ok(Height::new(hgt, units))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(day_4 =>
        (version: "1.0")
        (author: "Charles Srisuwananukorn <csrisuw@gmail.com>")
        (about: "advent of code day 4")
        (@arg INPUT: +required "input file")
    ).get_matches();

    let file = matches.value_of("INPUT").unwrap();

    let mut f = fs::File::open(file)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let expected_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let mut valid_keys = HashSet::new();

    let mut count = 0;

    for s in s.split(&[' ', '\n'][..]) {
        if s.len() == 0 {
            if expected_keys.iter().all(|&x| valid_keys.contains(x)) {
                count += 1;
            }
            valid_keys.clear();
        } else {
            let kv: Vec<_> = s.split(':').take(2).collect();
            let (key, value) = (kv[0], kv[1]);

            let valid = match key {
                "byr" => {
                    let val = value.parse::<u32>()?;
                    1920 <= val && val <= 2002
                },
                "iyr" => {
                    let val = value.parse::<u32>()?;
                    2010 <= val && val <= 2020
                },
                "eyr" => {
                    let val = value.parse::<u32>()?;
                    2020 <= val && val <= 2030
                },
                "hgt" => {
                    let result = value.parse::<Height>();
                    result.is_ok()
                },
                "hcl" => {
                    (value.len() == 7)
                        && (value.chars().nth(0).unwrap() == '#')
                        && (value.chars().skip(1).all(|ch| {
                            ('0' <= ch && ch <= '9')
                                || ('a' <= ch && ch <= 'f')
                        }))
                },
                "ecl" => {
                    valid_eye_colors.iter().any(|&s| s == value)
                },
                "pid" => {
                    (value.len() == 9)
                        && (value.chars().all(|ch| '0' <= ch && ch <= '9'))
                },
                _ => false,
            };

            println!("key={:?}, value={:?}, valid={:?}", key, value, valid);

            if valid {
                valid_keys.insert(key);
            }
        }
    }

    println!("{}", count);

    Ok(())
}
