use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

enum Object {
    Wall(Coord),
    Empty(Coord),
    Wind((Coord, Direction)),
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

struct Game {
    wind_state: HashSet<(Coord, Direction)>, // O(1) lookup
    wall_state: HashSet<Coord>,
    mx_x: i32,
    mx_y: i32,
}

impl Game {
    fn new(input: Vec<Object>) -> Game {
        let mut wind_state = HashSet::new();
        let mut wall_state = HashSet::new();
        for object in input {
            match object {
                Object::Wall(coord) => {
                    wall_state.insert(coord);
                }
                Object::Wind((coord, direction)) => {
                    wind_state.insert((coord, direction));
                }
                _ => (),
            }
        }

        let (mx_x, mx_y) = wall_state.iter().fold((0, 0), |acc, coord| {
            (acc.0.max(coord.x), acc.1.max(coord.y))
        });

        Game {
            wind_state: wind_state,
            wall_state: wall_state,
            mx_x: mx_x,
            mx_y: mx_y,
        }
    }

    fn run(
        &self,
        start: &Coord,
        end: &Coord,
        wind_state_mapper: &HashMap<i32, HashSet<Coord>>,
        round: i32,
    ) -> i32 {

        // carry out bfs, avoid going past route(wait instead)
        let mut seen = HashSet::<(Coord, i32)>::new();
        let mut queue = VecDeque::<(Coord, i32)>::new();
        queue.push_back((start.clone(), round));

        while !queue.is_empty() {
            if let Some((pos, round_taken)) = queue.pop_front() {
                let cur_wind_state = wind_state_mapper
                    .get(&round_taken)
                    .expect("wind state not found in mapper");
                if let false = seen.insert((pos, round_taken)) {
                    continue;
                }
                let possible_state = get_possible_states(cur_wind_state, &self.wall_state, pos);
                let found_exit_flag = possible_state
                    .iter()
                    .filter(|&coord| coord.eq(&end))
                    .count()
                    .ne(&0);
                match found_exit_flag {
                    true => {
                        return round_taken;
                    }
                    false => {
                        for coord in possible_state {
                            queue.push_back((coord, round_taken + 1));
                        }
                    }
                }
            };
        }
        panic!("No exit found");
    }
}

fn get_coord_hashset_from_hashset(wind_state: &HashSet<(Coord, Direction)>) -> HashSet<Coord> {
    let mut hashmap_to_return = HashSet::new();
    wind_state.iter().for_each(|(coord, _)| {
        hashmap_to_return.insert(*coord);
    });

    hashmap_to_return
}

fn get_wind_state_until(
    initial_wind_state: HashSet<(Coord, Direction)>,
    mx_x: i32,
    mx_y: i32,
    n: i32,
) -> HashMap<i32, HashSet<Coord>> {
    let mut wind_state_mapper = HashMap::new();
    wind_state_mapper.insert(0, get_coord_hashset_from_hashset(&initial_wind_state));
    let mut temp_wind_state = initial_wind_state;
    for index in 1..n {
        let updated_wind_state: HashSet<(Coord, Direction)> = temp_wind_state
            .iter()
            .map(|(coord, direction)| {
                let mut new_x = coord.x;
                let mut new_y = coord.y;
                match direction {
                    Direction::North => {
                        new_y = coord.y - 1;
                        if new_y == 0 {
                            new_y = mx_y - 1;
                        }
                    }
                    Direction::South => {
                        new_y = coord.y + 1;
                        if new_y == mx_y {
                            new_y = 1;
                        }
                    }
                    Direction::East => {
                        new_x = coord.x + 1;
                        if new_x == mx_x {
                            new_x = 1;
                        }
                    }
                    Direction::West => {
                        new_x = coord.x - 1;
                        if new_x == 0 {
                            new_x = mx_x - 1;
                        }
                    }
                }
                (Coord { x: new_x, y: new_y }, *direction)
            })
            .collect();
        wind_state_mapper.insert(index, get_coord_hashset_from_hashset(&updated_wind_state));
        temp_wind_state = updated_wind_state;
    }
    dbg!(wind_state_mapper.len());
    wind_state_mapper
}

// fn dbg_print_map(mx_x: i32, mx_y: i32, wind_state: &HashMap<Coord, Direction>) {
//     for y in 0..mx_y {
//         for x in 0..mx_x {
//             match wind_state.get(&Coord { x: x, y: y }) {
//                 Some(Direction::North) => print!("^"),
//                 Some(Direction::South) => print!("v"),
//                 Some(Direction::East) => print!(">"),
//                 Some(Direction::West) => print!("<"),
//                 None => print!("."),

//             }
//         }
//         println!();
//     }
// }

fn get_possible_states(
    wind_state: &HashSet<Coord>,
    wall_state: &HashSet<Coord>,
    pos: Coord,
) -> Vec<Coord> {
    // update wind_state
    // 4 case:
    // 1. x == 0 -> x = mx_x - 1
    // 2. x == mx_x -> x = 1
    // 3. y == 0 -> y = mx_y - 1
    // 4. y == mx_y -> y = 1

    let possible_movements = vec![(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)];
    let possible_state = possible_movements
        .iter()
        .map(|(dx, dy)| Coord {
            x: pos.x + dx,
            y: pos.y + dy,
        })
        .filter(|coord| !wall_state.contains(coord) && !wind_state.contains(coord))
        .collect::<Vec<Coord>>();

    possible_state
}

fn main() {
    let input = read_input("star.txt");
    let game = Game::new(input);
    let mx_x = game.mx_x;
    let mx_y = game.mx_y;
    let initial_wind_state = game.wind_state.clone();
    let wind_state_mapper = get_wind_state_until(initial_wind_state, mx_x, mx_y, 5000);
    let round_taken_by_round_one = game.run(
        &Coord { x: 1, y: 0 },
        &Coord {
            x: mx_x - 1,
            y: mx_y,
        },
        &wind_state_mapper,
        0,
    );
    // star 1 answer
    println!("round_taken_by_round_one: {}", round_taken_by_round_one);
    let round_taken_by_round_two = game.run(
        &Coord {
            x: game.mx_x - 1,
            y: game.mx_y,
        },
        &Coord { x: 1, y: 0 },
        &wind_state_mapper,
        round_taken_by_round_one,
    );
    println!("round_taken_by_round_two: {}", round_taken_by_round_two);
    let round_taken_by_round_three = game.run(
        &Coord { x: 1, y: 0 },
        &Coord {
            x: game.mx_x - 1,
            y: game.mx_y,
        },
        &wind_state_mapper,
        round_taken_by_round_two,
    );
    println!("round_taken_by_round_three: {}", round_taken_by_round_three);
}

fn read_input(filename: &str) -> Vec<(Object)> {
    let mut list_to_return = vec![];
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    // empty
                    // do nothing
                }
                '#' => {
                    // wall
                    let wall = Object::Wall(Coord {
                        x: x as i32,
                        y: y as i32,
                    });
                    list_to_return.push(wall)
                }
                '^' => {
                    // wind
                    let wind = Object::Wind((
                        Coord {
                            x: x as i32,
                            y: y as i32,
                        },
                        Direction::North,
                    ));
                    list_to_return.push(wind)
                }
                '<' => {
                    // wind
                    let wind = Object::Wind((
                        Coord {
                            x: x as i32,
                            y: y as i32,
                        },
                        Direction::West,
                    ));
                    list_to_return.push(wind)
                }
                '>' => {
                    // wind
                    let wind = Object::Wind((
                        Coord {
                            x: x as i32,
                            y: y as i32,
                        },
                        Direction::East,
                    ));
                    list_to_return.push(wind)
                }
                'v' => {
                    // wind
                    let wind = Object::Wind((
                        Coord {
                            x: x as i32,
                            y: y as i32,
                        },
                        Direction::South,
                    ));
                    list_to_return.push(wind)
                }
                _ => {
                    panic!("Unknown character: {}", c);
                }
            }
        }
    }

    // seal up top left and bottom right
    list_to_return.push(Object::Wall(Coord { x: 0, y: -1 }));
    list_to_return.push(Object::Wall(Coord { x: 1, y: -1 }));
    list_to_return.push(Object::Wall(Coord { x: 2, y: -2 }));

    list_to_return
}
