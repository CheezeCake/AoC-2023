use std::collections::HashMap;
use std::io;

fn find_calibration_value(s: &str, digits: &HashMap<&str, usize>) -> usize {
    let first = digits
        .iter()
        .map(|(digit_str, digit)| (s.find(digit_str), digit))
        .filter(|(index, _)| index.is_some())
        .min_by_key(|(index, _)| index.unwrap())
        .unwrap()
        .1;
    let last = digits
        .iter()
        .map(|(digit_str, digit)| (s.rfind(digit_str), digit))
        .filter(|(index, _)| index.is_some())
        .max_by_key(|(index, _)| index.unwrap())
        .unwrap()
        .1;

    first * 10 + last
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|line| line.unwrap()).collect();

    let digits: HashMap<&str, usize> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    println!(
        "part 1: {}",
        lines
            .iter()
            .map(|line| find_calibration_value(line, &digits))
            .sum::<usize>()
    );

    let digits_spelled: HashMap<&str, usize> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    println!(
        "part 2: {}",
        lines
            .iter()
            .map(|line| find_calibration_value(line, &digits_spelled))
            .sum::<usize>()
    );
}
