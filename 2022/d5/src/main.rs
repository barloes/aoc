use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read file
    let filename = "./input";
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

    // solution_1(first_half.clone(), second_half.clone());
    solution_2(first_half.clone(), second_half.clone());
}

#[derive(Debug)]
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

    fn pop(&mut self) -> Option<char> {
        return self.crates.pop();
    }

    fn peek(&self) -> Option<&char> {
        self.crates.last()
    }
}

fn solution_1(first_half: Vec<String>, second_half: Vec<String>) {
    let mut crate_list = get_crate_list(first_half);
    let instruction_list = get_instruction_list(second_half);

    for instruction in instruction_list {
        crate_list = execute_instruction(instruction, crate_list);
    }
    for cc in crate_list {
        match cc.peek() {
            None => {
                continue;
            }
            Some(c) => {
                print!("{}", c);
            }
        }
    }

}

fn solution_2(first_half: Vec<String>, second_half: Vec<String>) {
    let mut crate_list = get_crate_list(first_half);
    let instruction_list = get_instruction_list(second_half);

    for instruction in instruction_list {
        crate_list = execute_instruction_2(instruction, crate_list);
    }
    for cc in crate_list {
        match cc.peek() {
            None => {
                continue;
            }
            Some(c) => {
                print!("{}", c);
            }
        }
    }
}
fn get_crate_list(first_half: Vec<String>) -> Vec<CrateHandler> {
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
            Some(_) => {
                for i in (0..n - 1).rev() {
                    let c = first_half[i].chars().nth(index).unwrap();
                    if c.is_whitespace() {
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
        let number = iter.nth(1).unwrap().parse::<i32>().unwrap();
        let from = iter.nth(1).unwrap().parse::<i32>().unwrap();
        let to = iter.nth(1).unwrap().parse::<i32>().unwrap();
        instruction_list.push(Instruction { from, to, number });
    }

    println!("{:?}", instruction_list);
    instruction_list
}

fn execute_instruction_2(
    instruction: Instruction,
    mut crate_list: Vec<CrateHandler>,
) -> Vec<CrateHandler> {
    let Instruction { from, to, number } = instruction;

    let mut temp = VecDeque::<char>::new();
    for _i in 0..number {
        temp.push_front(crate_list[from as usize].pop().unwrap());
    }

    for _i in 0..number {
        crate_list[to as usize].push(temp.pop_front().unwrap());
    }

    crate_list
}
fn execute_instruction(
    instruction: Instruction,
    mut crate_list: Vec<CrateHandler>,
) -> Vec<CrateHandler> {
    let Instruction { from, to, number } = instruction;

    let mut temp = Vec::<char>::new();
    for i in 0..number {
        temp.push(crate_list[from as usize].pop().unwrap());
        crate_list[to as usize].push(temp[i as usize]);
    }

    crate_list
}
