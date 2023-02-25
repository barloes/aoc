use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Index,
};

fn main() {
    // read file
    let filename = "./test";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // get the crate on the top of each block

    // get index on reader.lines where line is empty
    let mut first_half = Vec::<String>::new();
    let mut second_half = Vec::<String>::new();

    let mut second_half_flag = false;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            second_half_flag = true;
            continue;
        }

        if second_half_flag {
            second_half.push(line);
        } else {
            first_half.push(line);
        }
    }

    solution_1(first_half, second_half);
}

struct Instruction {
    from: i32,
    to: i32,
    number: i32,
}

#[derive(Debug)]
struct CrateHandler {
    crates: Vec<char>,
}

impl CrateHandler {
    fn new() -> CrateHandler {
        CrateHandler {
            crates: Vec::<char>::new(),
        }
    }

    fn push(&mut self, some_crate: char) {
        self.crates.push(some_crate);
    }

    fn pop(&mut self) {
        self.crates.pop();
    }
    
}

fn solution_1(first_half: Vec<String>, second_half: Vec<String>) -> i32 {
    let crate_list = get_crate_list(first_half);
    let direction_list = get_instruction_list(second_half);

    1
}

fn get_crate_list(first_half: Vec<String>) -> Vec<CrateHandler> {
    // for line in first_half {
    //     println!("{} {}", line, line.len());
    // }
    let n = first_half.len();
    let m = first_half[0].len();
    let mut crate_list = Vec::<CrateHandler>::new();
    crate_list.push(CrateHandler::new());

    for index in 0..m {
        let c = first_half
            .last()
            .unwrap()
            .chars()
            .nth(index)
            .unwrap()
            .to_digit(10);

        let mut temp = CrateHandler::new();
        match c {
            None => {}
            Some(c) => {
                println!("found:{} {}", index, c);
                for i in (0..n - 1).rev() {
                    let c = first_half[i].chars().nth(index).unwrap();
                    if c.is_whitespace(){
                        break;
                    }
                    temp.push(c);
                }
                crate_list.push(temp);
            }
        }
    }
    crate_list
}

fn get_instruction_list(second_half: Vec<String>) -> Vec<Instruction> {
    let mut instruction_list = Vec::<Instruction>::new();
    for line in second_half {
        let mut iter = line.split_whitespace();
        // instruction_list.push(Instruction {
        //     from,
        //     to,
        //     number,
        // });
    }
    todo!();
    instruction_list
}