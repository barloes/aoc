use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);
    let mut ans = 0;
    // Iterate over each line in the file
    let mut counter = 0;
    let mut set1 = HashSet::<char>::new();
    let mut set2 = HashSet::<char>::new();
    for line in reader.lines(){
        match { line } {
            Ok(line) => {
                let line_str = line.to_string();
                let n = line_str.len();
                
                if counter % 3 == 0{
                    set1.clear();
                    set2.clear();
                    for i in 0..n {
                        set1.insert(line_str.chars().nth(i).unwrap());
                    }
                }
                else if counter % 3 == 1 {
                    for i in 0..n {
                        let c = line_str.chars().nth(i).unwrap();
                        if set1.contains(&c){
                            set2.insert(c);
                        }
                    }
                }
                else{
                    for i in 0..n {
                        let c = line_str.chars().nth(i).unwrap();
                        if set2.contains(&c) {
                            ans += get_char_score(c);
                        }
                        break;
                    } 
                }
            }
            Err(_) => {}
        }
        counter += 1;
    }      
    println!("{}", ans);
}

fn get_char_score(c: char) -> u32 {
    match c.is_uppercase() {
        true => c as u32 - 'A' as u32 + 1 + 26,
        false => c as u32 - 'a' as u32 + 1
    }
}