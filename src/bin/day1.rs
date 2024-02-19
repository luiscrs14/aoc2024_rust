use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let f = File::open("src/input/day1.txt").expect("file not found");
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let numbers_as_strings_re = Regex::new(
        r".*?(?P<first>\d|one|two|three|four|five|six|seven|eight|nine).*(?P<last>\d|one|two|three|four|five|six|seven|eight|nine)"
    ).unwrap();
    let single_number_re =
        Regex::new(r".*?(?P<first>\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut final_value: i32 = 0;

    for line in lines {
        let line = line.expect("Line should exist");
        let chars = line.chars();
        let chars = chars.as_str();

        let mut caps = numbers_as_strings_re.captures(chars);

        if caps.is_none() {
            caps = single_number_re.captures(chars);
        }
        let caps = caps.unwrap();

        let first_value = &caps.get(1).map_or("", |m| m.as_str());
        let last_value = &caps.get(2).map_or(first_value.to_owned(), |m| m.as_str());

        let first_value = convert_to_number(first_value);
        let last_value = convert_to_number(last_value);
        let line_value = first_value.to_owned() + last_value;
        final_value += line_value.parse::<i32>().unwrap();
        println!("Line: {line}, Value: {line_value}, Final value: {final_value}");
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Final value: {final_value}");
}

fn convert_to_number(value: &str) -> &str {
    match value {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => value,
    }
}
