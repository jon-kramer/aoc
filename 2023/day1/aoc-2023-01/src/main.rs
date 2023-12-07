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
                    let calibration = calculate_calibration(buffer.trim());
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
