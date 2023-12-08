use core::panic;
use std::io::stdout;
use std::io::{stdin, Write};

fn main() {
    let reader = stdin();
    let mut out = stdout().lock();
    let mut buffer = String::new();
    let mut sum = 0;

    loop {
        match reader.read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    let calibration = calculate_calibration2(buffer.trim());
                    sum += calibration;
                    writeln!(out, "{}", calibration).unwrap();
                    // let _ = out.flush();
                    buffer.clear();
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    writeln!(out, "{}", sum).unwrap();
}

// loop through characters and find the first number(n1) and the last number(n2) in the str
// then to find the final calibration, multiply the n1 by 10 and add to n2
fn calculate_calibration(line: &str) -> u32 {
    let mut found_num: bool = false;
    let mut n1: u32 = 10;
    let mut n2: u32 = 10;
    for char in line.chars() {
        if char.is_ascii_digit() {
            let num = char as u32 - '0' as u32;
            if !found_num {
                n1 = num;
                found_num = true;
            }
            n2 = num;
        }
    }

    if !found_num {
        panic!("BAD INPUT!")
    }

    (n1 * 10) + n2
}

/*
* "soneighthreeight"
*
* Loop through characters, checking first for ascii digits and then for substring matches
* against the valid string list.
*
* Once a substring is found, we can convert it to its numeric representation and set our n1,n2
* values appropriately.
*
* what about "oneight"? is the answer 11, 88, or 18?
* - for now this will be 18
*/

const WORD_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn calculate_calibration2(line: &str) -> u32 {
    let mut found_num: bool = false;
    let mut n1: u32 = 10;
    let mut n2: u32 = 10;
    for (i, char) in line.chars().enumerate() {
        // Attempt to find a digit, first checking ascii character, then checking for substring
        // matches
        let mut num: Option<u32> = None;
        if char.is_ascii_digit() {
            num = Some(char as u32 - '0' as u32);
        }
        // Get substring of buffer starting from this char and check if it begins with any of the DIGITS
        else {
            // get slice from index
            // for each DIGIT, begins_with, and if we find one, convert to num and set, else
            for (j, word_digit) in WORD_DIGITS.iter().enumerate() {
                if line[i..].starts_with(word_digit) {
                    num = Some(j as u32);
                }
            }
        }

        if let Some(n) = num {
            if !found_num {
                n1 = n;
                found_num = true;
            }
            n2 = n;
        }
    }

    if !found_num {
        panic!("BAD INPUT!")
    }

    (n1 * 10) + n2
}
