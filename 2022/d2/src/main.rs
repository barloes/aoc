use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);

    // Iterate over each line in the file

    let mut mx = 0;
    for line in reader.lines() {
        match { line } {
            Ok(line) => {
                let a: Vec<char> = line
                    .split(" ")
                    .map(|x| x.parse::<char>().unwrap())
                    .collect();
                let oppo_hand = a[0];
                let my_hand = a[1];
                // println!("oppo:{} hand:{} {}", oppo_hand, my_hand, get_expected_score(oppo_hand ,my_hand));
                let score = get_expected_score(oppo_hand ,my_hand);
                mx += score;
            }
            Err(_) => {}
        }
    }
    println!("{}", mx);
}

fn convert_to_score(hand: char) -> i32 {
    let result = match hand {
        'A' => 1,
        'X' => 1,
        'B' => 2,
        'Y' => 2,
        'C' => 3,
        'Z' => 3,
        _ => {panic!("idk")}
    };
    result
}

fn get_result_score(oppo_hand: i32, my_hand: i32) -> i32 {
    if oppo_hand == my_hand {
        return 3;
    }
    if (oppo_hand == 3 && my_hand == 1) || (oppo_hand + 1 == my_hand) {
        return 6;
    }

    0
}

fn get_winning_hand(oppo_hand: char) -> i32 {
    match oppo_hand {
        'A' => 2,
        'B' => 3,
        'C' => 1,
        _ => 0
    }
}

fn get_losing_hand(oppo_hand: char) -> i32 {
    match oppo_hand {
        'A' => 3,
        'B' => 1,
        'C' => 2,
        _ => 0
    }
}

fn get_expected_score(oppo_hand: char, expected: char) -> i32 {
    match expected {
        'X' => 0 + get_losing_hand(oppo_hand) ,
        'Y' => 3 + convert_to_score(oppo_hand),
        'Z' => 6 + get_winning_hand(oppo_hand),
        _ => 0
    }
}
