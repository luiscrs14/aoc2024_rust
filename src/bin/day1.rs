use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("src/input/day1.txt").expect("file not found");
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut final_value: i32 = 0;

    for line in lines {
        let mut first_value: char = '0';
        let mut last_value: char = '0'; // TODO this is potentially a bug
        let line = line.expect("Line should exist");
        let chars = line.chars();
        let rev_chars = line.chars().rev();
        for c in chars {
            if c.is_digit(10) {
                first_value = c;
                break;
            }
        }

        for c in rev_chars {
            if c.is_digit(10) {
                last_value = c;
                break;
            }
        }

        let mut line_value = String::new();
        line_value.push(first_value);
        line_value.push(last_value);
        final_value += line_value.parse::<i32>().unwrap();
    }

    println!("Final value: {final_value}");
}
