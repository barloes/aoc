use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct CalibrationValue {
    first: i32,
    last: i32,
}

fn main() {
    let result = read_file_part_two("test.txt");
    compute_result(result);
}

fn read_file_part_two(path: &str) -> Vec<CalibrationValue> {
    let mut calib = Vec::new();
    let file = File::open(path).expect("file not found");
    let reader = BufReader::new(file);

    let mut map_to_check = HashMap::new();
    map_to_check.insert("one", 1);
    map_to_check.insert("two", 2);
    map_to_check.insert("three", 3);
    map_to_check.insert("four", 4);
    map_to_check.insert("five", 5);
    map_to_check.insert("six", 6);
    map_to_check.insert("seven", 7);
    map_to_check.insert("eight", 8);
    map_to_check.insert("nine", 9);

    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        let mut numbers = Vec::new();
        for (index, char) in line_clone.char_indices() {
            for (key, value) in &map_to_check {
                if line_clone[0..index + 1].ends_with(key) {
                    numbers.push(value.clone());
                }
            }

            if char.is_numeric() {
                numbers.push(char.to_digit(10).unwrap() as i32);
            }
        }
        calib.push(CalibrationValue {
            first: numbers[0],
            last: numbers[numbers.len() - 1],
        })
    }
    calib
}

fn compute_result(calib: Vec<CalibrationValue>) {
    let mut result = 0;
    for value in calib {
        result += value.first * 10 + value.last;
    }

    println!("Result: {}", result);
}

fn read_file(path: &str) -> Vec<CalibrationValue> {
    let mut calib = Vec::new();
    let file = File::open(path).expect("file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut numbers = Vec::new();
        for char in line.unwrap().chars() {
            if char.is_numeric() {
                numbers.push(char.to_digit(10).unwrap() as i32);
            }
        }
        calib.push(CalibrationValue {
            first: numbers[0],
            last: numbers[numbers.len() - 1],
        })
    }
    calib
}
