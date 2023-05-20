use std::{
    collections::{HashMap, HashSet, VecDeque},
    f32::consts::E,
    fs, ops::{Deref, Add, Mul, Sub, Div},
};

use polynomial::Polynomial;

#[derive(Clone, Copy, Debug)]
enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
    EQUAL,
}

struct Result {
    operation: Option<Operation>,
    left: Option<String>,
    right: Option<String>,
    final_result: Option<FakeNumber>,
}

#[derive(Clone, Debug)]
struct Monkey {
    identifier: String,
    operation: Option<Operation>,
    left: Option<(String, Option<FakeNumber>)>,
    right: Option<(String, Option<FakeNumber>)>,
    final_result: Option<FakeNumber>,
}

// FakeNumber is holder for mx + c
// multiplier only affected by * and /
#[derive(Clone, Debug)]
struct FakeNumber{
    value: Polynomial<f64>,
}

impl FakeNumber {
    fn new(polynomial: Polynomial<f64>) -> FakeNumber {
        FakeNumber {
            value: polynomial,
        }
    }
}

// multiplier to take either side which is not None

impl Add for FakeNumber {
    type Output = Self;

    fn add(self, other: FakeNumber) -> Self {
        FakeNumber{
            value: self.value + other.value,
        }
    }
}

impl Mul for FakeNumber {
    type Output = Self;

    fn mul(self, other: FakeNumber) -> Self {
        FakeNumber{
            value: self.value * other.value,
        }
    }
}

impl Sub for FakeNumber {
    type Output = Self;

    fn sub(self, other: FakeNumber) -> Self {
        FakeNumber{
            value: self.value - other.value,
        }
    }
}
    
impl Div for FakeNumber {
    type Output = Self;

    fn div(self, other: FakeNumber) -> Self {
        // if either side is polynomial panic

        if self.value.data().len() > 1 && other.value.data().len() > 1 {
            dbg!(self.value);
            dbg!(other.value);
            panic!();
        }
        else if self.value.data().len() > 1 {
            let inverse = Polynomial::new(vec![1.0 / other.value.data()[0]]);
            return FakeNumber{
                value: self.value * inverse,
            }
        }
        else if other.value.data().len() > 1 {
            let mut new_inverse = vec![];
            for i in 0..other.value.data().len() {
                let new_inverse_coefficient = 1.0 / other.value.data()[i];
                new_inverse.push(new_inverse_coefficient);
            }
            let inverse = Polynomial::new(new_inverse);
            return FakeNumber{
                value: other.value * inverse,
            }
        } else {
            let other_inverse = Polynomial::new(vec![1.0 / other.value.data()[0]]);
            return FakeNumber{
                value: self.value * other_inverse,
            };
        }
    }   
}

impl Monkey {
    fn new(
        identifier: String,
        operation: Option<Operation>,
        left: Option<String>,
        right: Option<String>,
        final_result: Option<FakeNumber>,
    ) -> Monkey {
        let left_input = match left {
            Some(left) => Some((left, None)),
            None => None,
        };
        let right_input = match right {
            Some(right) => Some((right, None)),
            None => None,
        };
        Monkey {
            identifier: identifier,
            operation: operation,
            left: left_input,
            right: right_input,
            final_result: final_result,
        }
    }

    fn can_process(&self) -> bool {
        self.final_result.is_some()
    }

    fn update(&mut self, monkey: &Monkey) {
        if self.final_result.is_some() {
            return;
        }
        let left = self.left.as_mut().unwrap();
        if left.0 == monkey.clone().identifier {
            left.1 = Some(monkey.final_result.as_ref().unwrap().clone());
        }

        let right = self.right.as_mut().unwrap();
        if right.0 == monkey.clone().identifier {
            right.1 = Some(monkey.final_result.as_ref().unwrap().clone());
        }

        // self.left = Some(left.deref().clone());
        // self.right = Some(right.deref().clone());
        if self.left.as_ref().unwrap().1.is_some() && self.right.as_ref().unwrap().1.is_some() {
            self.final_result = match self.operation {
                Some(Operation::ADD) => {
                    Some(self.left.as_ref().unwrap().1.as_ref().unwrap().clone() + self.right.as_ref().unwrap().1.as_ref().unwrap().clone())
                }
                Some(Operation::SUB) => {
                    Some(self.left.as_ref().unwrap().1.as_ref().unwrap().clone() - self.right.as_ref().unwrap().1.as_ref().unwrap().clone())
                }
                Some(Operation::MUL) => {
                    Some(self.left.as_ref().unwrap().1.as_ref().unwrap().clone() * self.right.as_ref().unwrap().1.as_ref().unwrap().clone())
                }
                Some(Operation::DIV) => {
                    Some(self.left.as_ref().unwrap().1.as_ref().unwrap().clone() / self.right.as_ref().unwrap().1.as_ref().unwrap().clone())
                }
                Some(Operation::EQUAL) => {
                    // print left and right
                    // end it
                    dbg!(self.left.as_ref().unwrap().1.as_ref().unwrap());
                    dbg!(self.right.as_ref().unwrap().1.as_ref().unwrap());
                    panic!()
                }
                None => panic!(),
            };
        }
    }
}

struct Game {
    map: HashMap<String, Monkey>,
    update_map: HashMap<String, Vec<String>>,
}

impl Game {
    fn new(monkey_list: &Vec<Monkey>) -> Game {
        let mut map = HashMap::<String, Monkey>::new();
        let mut update_map = HashMap::<String, Vec<String>>::new();

        for monkey in monkey_list.clone() {
            map.insert(monkey.identifier.clone(), monkey.clone());

            if monkey.final_result.is_none() {
                let left_identifier = monkey.left.as_ref().unwrap().0.clone();
                let right_identifier = monkey.right.as_ref().unwrap().0.clone();
                update_map
                    .entry(left_identifier)
                    .or_insert(Vec::<String>::new())
                    .push(monkey.clone().identifier);
                update_map
                    .entry(right_identifier)
                    .or_insert(Vec::<String>::new())
                    .push(monkey.clone().identifier);
            }
        }

        Game {
            map: map,
            update_map: update_map,
        }
    }

    fn start(&mut self) -> i64 {
        // queue
        // initialise monkeys with no dependencies

        // on processing each monkey -> update those dependent on it -> if no longer dependant -> add to queue
        // add check for root
        let mut visited = HashSet::<String>::new();
        let mut queue = VecDeque::<Monkey>::new();
        self.map.iter().for_each(|(key, value)| {
            if value.can_process() {
                queue.push_back(value.clone());
            }
        });

        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap().clone();

            visited.insert(cur.clone().identifier);
            if cur.identifier == "root" {
                return self.get_monkey(&cur).unwrap().final_result.as_ref().unwrap().value.data()[0] as i64;
            }

            self.update_map.get(&cur.clone().identifier).unwrap().iter().for_each(|monkey_to_update_identifier| {
                let monkey_to_update = self.map.get_mut(monkey_to_update_identifier).unwrap();
                monkey_to_update.update(&cur);
                
                if monkey_to_update.can_process() && !visited.contains(&monkey_to_update.identifier)
                {
                    queue.push_back(monkey_to_update.clone());
                }
            });
        }

        panic!()
    }

    fn get_monkey(&self, monkey: &Monkey) -> Option<&Monkey> {
        let identifier = monkey.clone().identifier;
        return self.map.get(&identifier);
    }
}

fn main() {
    let monkey_list = read_input("input".to_string());
    let mut game = Game::new(&monkey_list);
    let solution_1_ans = game.start();
    println!("Solution 1: {}", solution_1_ans);

    let mut new_monkey_list = read_input("input1".to_string()); 
    let new_monkey = Monkey::new(
        "humn".to_string(),
        None,
        None,
        None,
        Some(FakeNumber::new(Polynomial::new(vec![0.0, 1.0])))
    );
    new_monkey_list.push(new_monkey);
    let mut game_2 = Game::new(&new_monkey_list);
    let solution_2_ans = game_2.start();
}

fn read_input(filename: String) -> Vec<Monkey> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut monkey_list = Vec::<Monkey>::new();
    for line in contents.lines() {
        let splitted = line.split(" ").collect::<Vec<&str>>();
        let identifier = splitted[0].replace(":", "");
        if splitted.len() == 2 {
            let final_result = splitted[1].parse::<f64>().unwrap();
            monkey_list.push(Monkey::new(
                identifier.to_string(),
                None,
                None,
                None,
                Some(FakeNumber::new(Polynomial::new(vec![final_result as f64]))),
            ));
        } else {
            let left_identifier = splitted[1];
            let operation = match splitted[2] {
                "+" => Operation::ADD,
                "-" => Operation::SUB,
                "*" => Operation::MUL,
                "/" => Operation::DIV,
                "=" => Operation::EQUAL,
                _ => panic!(),
            };
            let right_identifier = splitted[3];
            monkey_list.push(Monkey::new(
                identifier.to_string(),
                Some(operation),
                Some(left_identifier.to_string()),
                Some(right_identifier.to_string()),
                None,
            ));
        }
    }
    monkey_list
}
