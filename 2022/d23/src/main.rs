use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn get_all_neighbours(x: i32, y: i32) -> Vec<Coord> {
    let init_movement = vec![
        Coord { x: x - 1, y: y - 1 },
        Coord { x: x, y: y - 1 },
        Coord { x: x + 1, y: y - 1 },
        Coord { x: x + 1, y: y },
        Coord { x: x + 1, y: y + 1 },
        Coord { x: x, y: y + 1 },
        Coord { x: x - 1, y: y + 1 },
        Coord { x: x - 1, y: y },
    ];

    init_movement
}

// return top, left, right, bottom or all
fn validate_next_pos(
    x: i32,
    y: i32,
    movement: &Movement,
    neighbour_set: &HashSet<Coord>,
) -> Option<Coord> {
    let init_movement = get_all_neighbours(x, y);

    let movement_to_validate = match movement {
        Movement::Up => init_movement[0..3].to_vec(),
        Movement::Right => init_movement[2..5].to_vec(),
        Movement::Down => init_movement[4..7].to_vec(),
        Movement::Left => {
            let mut left = init_movement[6..8].to_vec();
            left.push(init_movement[0]);
            left
        }
    };

    // if there is no neighbour found in the path to go -> return Some(movement list)
    if movement_to_validate
        .iter()
        .filter(|&coord| neighbour_set.contains(coord))
        .count()
        .eq(&0)
    {
        let movement_to_go = match movement {
            Movement::Up => init_movement[1],
            Movement::Right => init_movement[3],
            Movement::Down => init_movement[5],
            Movement::Left => init_movement[7],
        };
        return Some(movement_to_go);
    } else {
        return None;
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Elf {
    coord: Coord,
    move_priority: Vec<Movement>,
    next_coord: Option<Coord>,
}

impl Elf {
    fn new(x: i32, y: i32) -> Elf {
        // let move_priority = vec![
        //     (0, 0 - 1),
        //     (0, 0 + 1),
        //     (0 - 1, 0),
        //     (0 + 1, 0),];

        let move_priority = vec![
            Movement::Up,
            Movement::Down,
            Movement::Left,
            Movement::Right,
        ];
        Elf {
            coord: Coord { x: x, y: y },
            move_priority: move_priority,
            next_coord: None,
        }
    }

    fn round_one(&mut self, neighbour_set: &HashSet<Coord>) {
        // only allow processing if there is neighbour
        let all_neighbour_coord = get_all_neighbours(self.coord.x, self.coord.y);

        // get the movement and updated it to next coord (only proceed if there is neighbour)
        if all_neighbour_coord
            .iter()
            .filter(|&coord| neighbour_set.contains(coord))
            .count()
            .gt(&0)
        {
            if let Some((index, movement)) = self
                .move_priority
                .iter()
                .enumerate()
                .filter(|&(_, movement)| {
                    validate_next_pos(self.coord.x, self.coord.y, &movement, neighbour_set)
                        .is_some()
                })
                .next()
            {
                let movement = movement.clone(); // take a copy of the movement
                self.next_coord =
                    validate_next_pos(self.coord.x, self.coord.y, &movement, neighbour_set);

                // if index + 1 < self.move_priority.len() {
                //     self.move_priority.rotate_left(index + 1);
                // }
            }
        }

        let first = self.move_priority.remove(0);
        self.move_priority.push(first);
    }

    // apply coord given that it is not in the set
    fn round_two(&mut self, coord_counter: &HashMap<Coord, i32>) {
        match self.next_coord {
            Some(coord) => {
                match coord_counter.get(&coord) {
                    Some(x) => {
                        if *x == 1 {
                            self.coord = coord;
                        }
                        // Reset it
                        self.next_coord = None;
                    }
                    None => {
                        panic!("coord have to be already counted in the counter");
                        // do nothing
                    }
                }
            }
            None => {
                // do nothing
            }
        }
    }
}

struct Game {
    elf_list: Vec<Elf>,
}

impl Game {
    fn new(grid_list: Vec<((i32, i32), Grid)>) -> Game {
        let mut grid_map = HashMap::new();
        for ((x, y), grid) in grid_list.iter() {
            grid_map.insert((x.clone(), y.clone()), grid.clone());
        }
        let init_elf = grid_map
            .iter()
            .filter_map(|(&coord, &grid_type)| match grid_type {
                Grid::Elf => Some(Elf::new(coord.0, coord.1)),
                _ => None,
            })
            .collect::<Vec<Elf>>();
        Game { elf_list: init_elf }
    }

    fn play_n_rounds(&mut self, n: i32) {
        self.debug_print();
        for _ in 0..n {
            // recompute coord set every round
            let elves_coord_set = self
                .elf_list
                .iter()
                .map(|elf| elf.coord)
                .collect::<HashSet<Coord>>();

            let mut counter = HashMap::new();
            for elf in self.elf_list.iter_mut() {
                elf.round_one(&elves_coord_set);
                // remove error on unwrap with None -> panic
                if let Some(coord) = elf.next_coord {
                    let count = counter.entry(coord).or_insert(0);
                    *count += 1;
                }
            }

            // dbg!("after round 1");
            // dbg!(&self.elf_list);

            for elf in self.elf_list.iter_mut() {
                elf.round_two(&counter);
            }

            self.debug_print();
        }
    }

    fn round_2_play_until_no_movement(&mut self) {
        let n = 100000;
        for round in 0..n {
            // recompute coord set every round
            let elves_coord_set = self
                .elf_list
                .iter()
                .map(|elf| elf.coord)
                .collect::<HashSet<Coord>>();

            let mut counter = HashMap::new();
            for elf in self.elf_list.iter_mut() {
                elf.round_one(&elves_coord_set);
                // remove error on unwrap with None -> panic
                if let Some(coord) = elf.next_coord {
                    let count = counter.entry(coord).or_insert(0);
                    *count += 1;
                }
            }

            for elf in self.elf_list.iter_mut() {
                elf.round_two(&counter);
            }
            let elves_coord_set_end_round = self
                .elf_list
                .iter()
                .map(|elf| elf.coord)
                .collect::<HashSet<Coord>>();
            if elves_coord_set == elves_coord_set_end_round {
                println!("round: {}", round + 1);
                return;
            }
        }
    }

    fn debug_print(&self) {
        let grid_list = self.elf_list.iter().collect::<Vec<_>>();
        let grid_hashset = self
            .elf_list
            .iter()
            .map(|elf| elf.coord)
            .collect::<HashSet<_>>();
        // let (mut mn_x, mut mn_y, mut mx_x, mut mx_y) = (
        //     grid_list[0].coord.x,
        //     grid_list[0].coord.y,
        //     grid_list[0].coord.x,
        //     grid_list[0].coord.y,
        // );
        // for (&elf) in grid_list.iter() {
        //     let x = elf.coord.x;
        //     let y = elf.coord.y;
        //     if x < mn_x {
        //         mn_x = x;
        //     }
        //     if y < mn_y {
        //         mn_y = y;
        //     }
        //     if x > mx_x {
        //         mx_x = x;
        //     }
        //     if y > mx_y {
        //         mx_y = y;
        //     }
        // }
        let mn_x = 0;
        let mn_y = 0;
        let mx_x = 13;
        let mx_y = 11;

        for y in mn_y..=mx_y {
            for x in mn_x..=mx_x {
                match grid_hashset.get(&Coord { x: x, y: y }) {
                    Some(_) => {
                        print!("#");
                    }
                    None => {
                        print!(".");
                    }
                    _ => {
                        panic!("should be empty or elf")
                    }
                }
            }
            println!();
        }
        println!();
    }

    // for getting answer of solution one
    fn print_star_one_ans(&self) {
        let grid_list = self.elf_list.iter().collect::<Vec<_>>();
        let grid_hashset = self
            .elf_list
            .iter()
            .map(|elf| elf.coord)
            .collect::<HashSet<_>>();
        let (mut mn_x, mut mn_y, mut mx_x, mut mx_y) = (
            grid_list[0].coord.x,
            grid_list[0].coord.y,
            grid_list[0].coord.x,
            grid_list[0].coord.y,
        );
        for (&elf) in grid_list.iter() {
            let x = elf.coord.x;
            let y = elf.coord.y;
            if x < mn_x {
                mn_x = x;
            }
            if y < mn_y {
                mn_y = y;
            }
            if x > mx_x {
                mx_x = x;
            }
            if y > mx_y {
                mx_y = y;
            }
        }
        println!("x: {}", mx_x + 1 - mn_x);
        println!("y: {}", mx_y + 1 - mn_y);
        println!(
            "ans: {}",
            (mx_x + 1 - mn_x) * (mx_y + 1 - mn_y) - self.elf_list.len() as i32
        );
    }
}

fn main() {
    let grid_list = handle_input();
    let mut game = Game::new(grid_list);
    // game.play_n_rounds(11);

    game.round_2_play_until_no_movement();
}

#[derive(Copy, Clone)]
enum Grid {
    Elf,
    Empty,
}

fn handle_input() -> Vec<((i32, i32), Grid)> {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut grid_list = Vec::new();
    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    // empty
                    grid_list.push(((x as i32, y as i32), Grid::Empty));
                }
                '#' => {
                    // elf
                    grid_list.push(((x as i32, y as i32), Grid::Elf));
                }
                _ => {
                    panic!("Unknown character: {}", c);
                }
            }
        }
    }
    grid_list
}
