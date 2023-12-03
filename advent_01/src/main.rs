extern crate log;
use std::fs::read_to_string;

fn main() {
    let lines = read_lines("./src/input.txt");

    let r: u32 = lines
        .iter()
        .map(|l| filter_numeric(l))
        .map(|l| first_and_last(&l))
        .sum();

    println!("{:?}", r);
}

fn filter_numeric(string: &str) -> String {
    string.chars().filter(|c| !c.is_alphabetic()).collect()
}

fn first_and_last(string: &str) -> u32 {
    let chars = string.chars();
    let first = chars.clone().next().unwrap();
    let last = chars.last();

    match last {
        Some(x) => [first, x]
            .into_iter()
            .collect::<String>()
            .parse::<u32>()
            .unwrap(),
        None => first as u32,
    }
}

fn read_lines(file: &str) -> Vec<String> {
    read_to_string(file)
        .expect("Cannot read file")
        .lines()
        .map(String::from)
        .collect()
}
