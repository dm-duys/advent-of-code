use fancy_regex::Regex;
use std::{collections::HashMap, fs};

const _EXAMPLE: &str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

pub fn day_one() {
    let mut combined_total = 0;
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;

    let filename = "day_1.txt";

    if let Ok(contents) = fs::read_to_string(filename) {
        for line in contents.lines() {
            first_digit = None;
            last_digit = None;

            for c in line.chars() {
                if c.is_digit(10) {
                    if first_digit.is_none() {
                        first_digit = Some(c);
                    }
                    last_digit = Some(c);
                }
            }
            if first_digit.is_some() {
                let combined_digits = format!(
                    "{}{}",
                    first_digit.unwrap(),
                    last_digit.unwrap_or(first_digit.unwrap())
                );
                let combined_digits: u32 = combined_digits.parse().unwrap();
                combined_total += combined_digits;
            }
        }
    }

    println!("Day One: combined_total: {}", combined_total);
}

pub fn day_one_part_two() {
    let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|[0-9]))").unwrap();
    let mut combined_total: u32 = 0;
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;

    let filename = "day_2.txt";

    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
    ];
    let numbers = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

    let map: HashMap<_, _> = words.iter().zip(numbers.iter()).collect();

    if let Ok(contents) = fs::read_to_string(filename) {
        for line in contents.lines() {
            first_digit = None;
            last_digit = None;

            for i in 0..line.len() {
                if let Some(cap) = re.captures(&line[i..]).unwrap() {
                    let reg_match: &str = &cap[1];
                    // probably a single digit convert to char
                    if reg_match.len() == 1 {
                        let digit = reg_match.chars().next().unwrap();
                        if first_digit.is_none() {
                            first_digit = Some(digit);
                        }
                        last_digit = Some(digit);
                    } else {
                        // most likely a word
                        let digit = map.get(&reg_match).unwrap();
                        if first_digit.is_none() {
                            first_digit = Some(digit.chars().next().unwrap());
                        }
                        last_digit = Some(digit.chars().next().unwrap());
                    }
                    // i += reg_match.len() - 1;
                    print!("{:<10} ", reg_match);
                }
            }
            println!();
            if first_digit.is_some() {
                let combined_digits = format!(
                    "{}{}",
                    first_digit.unwrap(),
                    last_digit.unwrap_or(first_digit.unwrap())
                );
                let combined_digits: u32 = combined_digits.parse().unwrap();

                combined_total += combined_digits;
                println!(
                    "line: {:<60} combined_digits: {}, running total: {}",
                    line, combined_digits, combined_total
                );
            };
        }
    }
    println!("Day One Part 2: combined_total: {}", combined_total);
}
