use regex::Regex;
use std::fs::read_to_string;

fn process_input() -> String {
    read_to_string("inputs/third/input.txt").expect("Error getting input")
}

pub fn find_sum_regex() -> i32 {
    let text = process_input();

    let regex = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();

    let sum = regex
        .captures_iter(text.as_str())
        .map(|cap| cap.extract().1)
        .map(|[a, b]| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum();

    println!("Total sum: {sum}");
    sum
}
