use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up = 3,
    Right = 0,
    Down = 1,
    Left = 2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
    Wall,
    Path,
    Void,
}

impl Direction {
    fn get_clockwise(&self) -> Direction {
        let new_value = ((*self as i32) + 1) % 4;
        get_direction(new_value)
    }

    fn get_counter_clockwise(&self) -> Direction {
        let new_value = ((*self as i32) - 1 + 4) % 4;
        get_direction(new_value)
    }
}

fn get_direction(value: i32) -> Direction {
    match value {
        3 => Direction::Up,
        0 => Direction::Right,
        1 => Direction::Down,
        2 => Direction::Left,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}


impl Position {
    fn change_direction(&mut self, is_clockwise: bool) {
        self.direction = if is_clockwise {
            self.direction.get_clockwise()
        } else {
            self.direction.get_counter_clockwise()
        }
    }

    fn get_next_position(&self, boundary: (i32, i32, i32, i32)) -> Position {
        let mut new_position = self.clone();
        match self.direction {
            Direction::Up => new_position.y -= 1,
            Direction::Down => new_position.y += 1,
            Direction::Left => new_position.x -= 1,
            Direction::Right => new_position.x += 1,
        }
        if new_position.x < boundary.0 {
            new_position.x = boundary.1;
        } else if new_position.x > boundary.1 {
            new_position.x = boundary.0;
        } else if new_position.y < boundary.2 {
            new_position.y = boundary.3;
        } else if new_position.y > boundary.3 {
            new_position.y = boundary.2;
        }
        new_position
    }
}

trait Instruction {
    fn execute(&self, position: &Position, position_helper: &HashMap<(Position), (i32, i32)>) -> Position;
}

#[derive(Debug)]
struct MoveMechanic {
    magnitude: i32,
}

#[derive(Debug)]
struct TurnMechanic {
    is_clockwise: bool,
}

impl Instruction for MoveMechanic {
    // return final position
    fn execute(&self, position: &Position, position_helper: &HashMap<Position, (i32, i32)>) -> Position {
        dbg!(self.magnitude);
        let mut final_position = position.clone();
        for _ in 0..self.magnitude {
            let mut new_position = final_position.clone();
            dbg!(new_position.clone());
            final_position = match position_helper.get(&new_position) {
                Some((x, y)) => {
                    new_position.x = x.clone();
                    new_position.y = y.clone();
                    new_position
                },
                None => {
                    break;
                }
            }
        }
        final_position
    }
}

impl Instruction for TurnMechanic {
    fn execute(&self, position: &Position, map: &HashMap<Position, (i32, i32)>) -> Position {
        let mut final_position = position.clone();
        final_position.change_direction(self.is_clockwise);
        final_position
    }
}

struct Map {
    map: HashMap<(i32, i32), Block>,
    position_helper: HashMap<(Position), (i32, i32)>,
}

impl Map {
    fn new(mut matrix:HashMap<(i32, i32), Block>) -> Map {
        let mut position_helper = HashMap::new();
        let min_x = matrix.iter().map(|((x, _), _)| *x).min().unwrap();
        let max_x = matrix.iter().map(|((x, _), _)| *x).max().unwrap();
        let min_y = matrix.iter().map(|((_, y), _)| *y).min().unwrap();
        let max_y = matrix.iter().map(|((_, y), _)| *y).max().unwrap();
        // fill up with void
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if !matrix.contains_key(&(x, y)) {
                    matrix.insert((x, y), Block::Void);
                }
            }
        }


        // if next is a path, add to hashmap
        // else if next is a wall. add None
        // else if next is a void, continue to next until ^

        matrix.iter().for_each(|((x, y), block)| {
            if block == &Block::Path {
                for direction in 0..=3 {
                    let mut checking_position = Position {
                        x: x.clone(),
                        y: y.clone(),
                        direction: get_direction(direction),
                    };
                    loop {
                        let next_position = checking_position.get_next_position((min_x, max_x, min_y, max_y));
                        match matrix.get(&(next_position.x, next_position.y)) {
                            Some(Block::Path) => {
                                position_helper.insert(
                                    Position {
                                        x: x.clone(),
                                        y: y.clone(),
                                        direction: get_direction(direction),
                                    },
                                    (next_position.x, next_position.y),
                                );
                                break;
                            }
                            Some(Block::Wall) => {
                                break;
                            }
                            Some(Block::Void) => {
                                checking_position = next_position;
                            }
                            None => {
                                unreachable!()
                            }
                        }
                    }
                }
            }
        });
        
        Map {
            map: matrix,
            position_helper: position_helper,
        }
    }

    fn start(&self, instruction_list: Vec<Box<dyn Instruction>>) -> i32 {
        let mut position = Position {
            x: self
                .map
                .iter()
                .filter(|&(&(x, y), block)| y == 0 && *block == Block::Path)
                .map(|((x, _), _)| *x)
                .min()
                .unwrap(),
            y: 0,
            direction: Direction::Right,
        };

        for instruction in instruction_list {
            position = instruction.execute(&position, &self.position_helper);
        }

        (position.y + 1) * 1000 + (position.x + 1) * 4 + (position.direction as i32)
    }
}

fn main() {
    let (mut map, instruction_list) = read_input("input".to_string());
    let game = Map::new(map);
    let result = game.start(instruction_list);
    println!("{}", result);
}

fn read_input(filename: String) -> (HashMap<(i32, i32), Block>, Vec<Box<dyn Instruction>>) {
    let mut file = File::open(filename).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let splitted = content.split("\n\n").collect::<Vec<&str>>();

    let map_content = splitted[0];
    let instruction_content = splitted[1];
    let mut map = HashMap::new();

    for (y,line) in map_content.lines().enumerate() {
        for x in 0..line.len() {
            let block = match line.chars().nth(x).unwrap() {
                '#' => Block::Wall,
                '.' => Block::Path,
                ' ' => Block::Void,
                _ => unreachable!(),
            };
            println!("x: {}, y: {}, block: {:?}", x, y, block);
            map.insert((x as i32, y as i32), block);
        }
    }
    let mut instruction_list = Vec::new();
    let mut start = 0;
    for index in 0..instruction_content.len() {
        if instruction_content.chars().nth(index).unwrap().is_alphabetic(){
            let magnitude = Box::new(MoveMechanic{magnitude: instruction_content[start..index].parse::<i32>().unwrap()}) as Box<dyn Instruction>;
            let turn_instruction = match instruction_content.chars().nth(index).unwrap() {
                'R' => Box::new(TurnMechanic{is_clockwise: true}) as Box<dyn Instruction>,
                'L' => Box::new(TurnMechanic{is_clockwise: false}) as Box<dyn Instruction>,
                _ => unreachable!(),
            };
            instruction_list.push(magnitude);
            instruction_list.push(turn_instruction);

            start = index + 1;
        }
    }
    if start < instruction_content.len() {

        let magnitude = Box::new(MoveMechanic{magnitude: instruction_content[start..].parse::<i32>().unwrap()}) as Box<dyn Instruction>;
        instruction_list.push(magnitude);
    }

    (map, instruction_list)
}

fn init_cube() {

    // create cubes
    
    // paint outward facing side
    // combine the cubes
    // if not in hashmap. move around the cube instead ?    
}