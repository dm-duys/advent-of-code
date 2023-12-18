use std::fs;

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

    let filename = "input.txt";

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
