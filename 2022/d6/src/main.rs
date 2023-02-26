use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn main() {
    // read file
    let filename = "./input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let char_list = reader
        .lines()
        .flatten()
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>();

    // solution_1(char_list, 4);

    solution_1(char_list, 14);
}

fn solution_1(char_list: Vec<char>, n: usize) {
    let mut sliding_window = HashSet::<char>::new();
    let mut start = 0;

    for end in 0..char_list.len() {
        // move start to the index+1 of the first duplicate
        if sliding_window.contains(&char_list[end]) {
            while sliding_window.contains(&char_list[end]) {
                sliding_window.remove(&char_list[start]);
                start += 1;
            }
        }

        sliding_window.insert(char_list[end]);
        if end - start + 1 == n {
            println!("ans: {}", end + 1);
            break;
        }
    }
}
