use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

#[derive(Clone, Debug, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Square,
}

#[derive(Clone, Debug)]
struct OperationImpl {
    operation: Operation,
    x: String,
}

impl OperationImpl {
    fn new(operation: String, x: String) -> OperationImpl {
        if x == "old" {
            return OperationImpl {
                operation: Operation::Square,
                x: x,
            };
        }
        // convert String to &str
        let operation_adr = &operation[..];
        match operation_adr {
            "+" => {
                return OperationImpl {
                    operation: Operation::Add,
                    x: x,
                }
            }
            "-" => {
                return OperationImpl {
                    operation: Operation::Sub,
                    x: x,
                }
            }
            "*" => {
                return OperationImpl {
                    operation: Operation::Mul,
                    x: x,
                }
            }
            "/" => {
                return OperationImpl {
                    operation: Operation::Div,
                    x: x,
                }
            }
            _ => {
                panic!()
            }
        }
    }

    fn execute(&self, a: i128) -> i128 {
        let b = self.x.clone();
        match self.operation {
            Operation::Add => a + b.parse::<i128>().unwrap(),
            Operation::Sub => a - b.parse::<i128>().unwrap(),
            Operation::Mul => a * b.parse::<i128>().unwrap(),
            Operation::Div => a / b.parse::<i128>().unwrap(),
            Operation::Square => a * a,
        }
    }
}

#[derive(Clone, Debug, Copy)]
struct Test {
    divisible: i128,
    monkey_1: i128,
    monkey_2: i128,
}

impl Test {
    fn new(divisible: i128, monkey_1: i128, monkey_2: i128) -> Test {
        Test {
            divisible,
            monkey_1,
            monkey_2,
        }
    }

    fn get_monkey_identifier(&self, item: i128) -> (i128) {
        if item % self.divisible == 0 {
            self.monkey_1
        } else {
            self.monkey_2
        }
    }

    fn get_divisible(&self) -> i128 {
        self.divisible
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    item_list: VecDeque<i128>,
    operation: OperationImpl,
    test: Test,
    inspect_count: i128,
}

impl Monkey {
    fn new(item_list: VecDeque<i128>, operation: OperationImpl, test: Test) -> Monkey {
        Monkey {
            item_list: item_list,
            operation,
            test,
            inspect_count: 0,
        }
    }

    fn add_item(&mut self, item: i128) {
        self.item_list.push_back(item);
    }

    fn pop(&mut self) -> i128 {
        self.item_list.pop_front().unwrap()
    }
}

#[derive(Clone, Debug)]
struct Game {
    monkey_map: HashMap<i128, RefCell<Monkey>>,
}

impl Game {
    fn new() -> Game {
        Game {
            monkey_map: HashMap::new(),
        }
    }

    fn get_divisible_lcm(&self) -> i128 {
        self
            .monkey_map
            .values()
            .map(|x| x.borrow().test.get_divisible())
            .product()
    }

    fn execute(&mut self, n: i128) {
        // get current monkey item from item list

        for i in 0..self.monkey_map.keys().len() {
            let identifier = i as i128;
            // println!("{:?}", self.monkey_map.get(&identifier).unwrap().borrow());

            while self
                .monkey_map
                .get(&identifier)
                .unwrap()
                .borrow()
                .item_list
                .len()
                > 0
            {
                // println!("monkey: {}, inspect_count: {}", identifier, self.monkey_map.get(&identifier).unwrap().borrow().inspect_count);
                let item_to_move = self.monkey_map.get(&identifier).unwrap().borrow_mut().pop();
                self.monkey_map
                    .get(&identifier)
                    .unwrap()
                    .borrow_mut()
                    .inspect_count += 1;

                let new_item_value = self
                    .monkey_map
                    .get(&identifier)
                .unwrap()
                    .borrow()
                    .operation
                    .execute(item_to_move)
                    / n % self.get_divisible_lcm();

                let monkey_to_pass = self
                    .monkey_map
                    .get(&identifier)
                    .unwrap()
                    .borrow()
                    .test
                    .get_monkey_identifier(new_item_value);

                // println!("item_to_move: {}, new_item_value: {}, monkey_to_pass: {}", item_to_move, new_item_value, monkey_to_pass);
                self.monkey_map
                    .get(&monkey_to_pass)
                    .unwrap()
                    .borrow_mut()
                    .add_item(new_item_value);
            }
        };
    }

    fn add_monkey(&mut self, content: String) {
        let (identifier, monkey) = parse_monkey_block(content);

        self.monkey_map.insert(identifier, RefCell::new(monkey));
    }

    fn get_solution_1(&self) {
        // sort monkey map by inspect count
        let mut monkey_map = self.monkey_map.clone();
        let mut monkey_list = monkey_map
            .values()
            .map(|x| x.borrow().clone())
            .collect::<Vec<Monkey>>();
        monkey_list.sort_by(|a, b| a.inspect_count.cmp(&b.inspect_count));

        // get last 2 monkey
        let monkey_1 = monkey_list.pop().unwrap();
        let monkey_2 = monkey_list.pop().unwrap();
        println!(
            "solution 1: {}",
            monkey_1.inspect_count * monkey_2.inspect_count
        );
    }

    fn print(&self) {
        for (identifier, monkey) in &self.monkey_map {
            println!("{}: {:?}", identifier, monkey.borrow().item_list);
        }
    }
    fn print_inpected_count(&self) {
        for i in 0..self.monkey_map.keys().len() {
            let identifier = i as i128;
            println!(
                "monkey: {}, inspect_count: {}",
                identifier,
                self.monkey_map.get(&identifier).unwrap().borrow().inspect_count
            );
        }
    }
}

fn main() {
    let mut game = Game::new();
    // read file
    let file = std::fs::read_to_string("input").unwrap();
    let monkey_block_list = file.split("\n\n");
    for monkey_block in monkey_block_list {
        game.add_monkey(monkey_block.to_string());
    }

    // for i in 0..20 {
    //     game.execute(3);
    //     println!("round {}:", i);
    //     game.print_inpected_count();
    // }

    for i in 0..10000 {
        game.execute(1);
        println!("round {}:", i);
        game.print_inpected_count();
        game.print();
    }
    game.get_solution_1();
}

fn parse_monkey_block(monkey_block: String) -> (i128, Monkey) {
    let mut monkey_lines = monkey_block.split("\n");
    let line_1 = monkey_lines.nth(0).unwrap();
    let mut monkey_identifier = line_1
        .split("Monkey")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .nth(0)
        .unwrap()
        .to_string()
        .split(":")
        .nth(0)
        .unwrap()
        .parse::<i128>()
        .unwrap();
    // println!("monkey identifier: {}", monkey_identifier);

    let line_2 = monkey_lines.nth(0).unwrap();
    let mut monkey_item_list = line_2
        .split("Starting items:")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse::<i128>().unwrap())
        .collect::<VecDeque<i128>>();
    // println!("{:?}", monkey_item_list);

    let line_3 = monkey_lines.nth(0).unwrap();
    // println!("{}", line_3);
    let mut operation_content = monkey_block
        .split("Operation: new = old ")
        .nth(1)
        .unwrap()
        .split_whitespace();

    let op = operation_content.nth(0).unwrap().trim();
    let x = operation_content.nth(0).unwrap().trim();
    let operation = OperationImpl::new(op.to_string(), x.to_string());

    let line_4 = monkey_lines.nth(0).unwrap();
    let divisible = line_4
        .split("Test: divisible by ")
        .nth(1)
        .unwrap()
        .trim()
        .parse::<i128>()
        .unwrap();

    let line_5 = monkey_lines.nth(0).unwrap();
    let monkey_1 = line_5
        .split("If true: throw to monkey ")
        .nth(1)
        .unwrap()
        .trim()
        .parse::<i128>()
        .unwrap();

    let line_6 = monkey_lines.nth(0).unwrap();
    let monkey_2 = line_6
        .split("If false: throw to monkey ")
        .nth(1)
        .unwrap()
        .trim()
        .parse::<i128>()
        .unwrap();
    let test = Test::new(divisible, monkey_1, monkey_2);

    // println!("{}", monkey_2);

    let mut monkey = Monkey::new(monkey_item_list, operation, test);

    (monkey_identifier, monkey)
}

fn run_operation(op: Operation, a: i128, b: String) -> i128 {
    match op {
        Operation::Add => a + b.parse::<i128>().unwrap(),
        Operation::Sub => a - b.parse::<i128>().unwrap(),
        Operation::Mul => a * b.parse::<i128>().unwrap(),
        Operation::Div => a / b.parse::<i128>().unwrap(),
        Operation::Square => a * a,
    }
}
