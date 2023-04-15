
/*
input: 
jetstream pattern in < and >, repeat if finished

output:
height after 2022 rock

5 rock pattern

chamber 
 - width 7 units

rock spawning
 - 2 units from the left edge
 - 3 units above the highest rock

mechanics
 - push by jetstream
 - falling one unit down
    - if rock stop (no more movement) -> spawn new rock 
 */

/*
problems:
 - game mechanics cycle
    - spawn rock
    - move rock (include getting pushed by jetstream and stopping)
 - how to detect collision
    - ?
 */

use std::{collections::{HashMap, HashSet, BTreeSet}, cmp::max};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rock {
    coords: Vec<Vec<(i64, i64)>>,
    index: usize
}

impl Rock {
    fn new(index: i64) -> Rock {

        let mut vec_to_add = vec![];
        vec_to_add.push(vec![(0, 0), (1, 0), (2, 0), (3, 0)]);
        vec_to_add.push(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]);
        vec_to_add.push(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]);
        vec_to_add.push(vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
        vec_to_add.push(vec![(0, 0), (1, 0), (0, 1), (1, 1)]);

        Rock {
            coords: vec_to_add,
            index: 0
        }
    }

    fn next(&mut self) -> Vec<(i64, i64)> {
        let result = self.coords[self.index].clone();
        self.index = (self.index + 1) % self.coords.len();
        return result;
    }

    fn get(&self) -> Vec<(i64, i64)> {
        return self.coords[self.index].clone();
    }

    fn get_index(&self) -> i64 {
        return self.index as i64;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Jetstream {
    positions: Vec<i64>,
    index: usize
}

impl Jetstream {
    fn new(input: String) -> Jetstream {
        let mut positions = Vec::new();
        for c in input.chars() {
            match c {
                '<' => positions.push(-1),
                '>' => positions.push(1),
                _ => panic!("invalid input")
            }
        }
        return Jetstream {
            positions: positions,
            index: 0
        }
    }

    fn next(&mut self) -> i64 {
        let result = self.positions[self.index];
        self.index = (self.index + 1) % self.positions.len();
        return result;
    }

    fn get_index(&self) -> i64 {
        return self.index as i64;
    }    
}

struct Game {
    highest_height: i64, // starting height for spawing new rock
    map: HashSet<(i64, i64)> // map of all taken spot
}

impl Game {
    fn new() -> Game {
        return Game {
            highest_height: 0,
            map: HashSet::new()
        }
    }

    fn print_map_last_n(&self, n: i64) {
        let mut map = self.map.clone();
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for (x, y) in &map {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        for y in (max_y-(n - 1)..=max_y).rev() {
            for x in 0..=6 {
                if map.contains(&(x, y)) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn spawn_rock(&self, rock: Vec<(i64, i64)>) -> Vec<(i64,i64)>{

        let x = 2;
        let y = self.highest_height + 3;
        return rock.iter().map(|coord| (coord.0 + x, coord.1 + y)).collect()
        
    }

    fn move_rock_by_jetstream(&self, rock: &Vec<(i64, i64)>, jetstream: &mut Jetstream) -> Vec<(i64, i64)> {
        let position_to_move = jetstream.next();
        return rock.iter().map(|coord| (coord.0 + position_to_move, coord.1)).collect();
    }

    fn move_rock_by_gravity(&self, rock: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {   
        return rock.iter().map(|coord| (coord.0, coord.1 - 1)).collect();
    }

    fn is_out_of_bound(&self, rock: &Vec<(i64, i64)>, height: i64) -> bool {
        for coord in rock {
            let x = coord.0;
            let y = coord.1;
            if x < 0 || x > 6 || y < height {
                return true;
            }
        }
        return false;
    }

    fn is_collided(&self, rock: &Vec<(i64, i64)>) -> bool {
        for coord in rock {
            let x = coord.0;
            let y = coord.1;
            if self.map.contains(&(x, y)) {
                return true;
            }
        }
        return false;
    }

    fn move_n_number_of_rocks(&mut self, jetstream: &mut Jetstream, n: i64) -> i64 {
        let mut memo = HashMap::<(i64, i64, i64), (i64, i64)>::new();
        let mut lazy = HashMap::<i64, i64>::new();
        let min_height = 0;
        let mut rock_factory = Rock::new(0);
        let mut index = 1;
        while index <= n {
            let rock = rock_factory.next();
            let mut starting_rock = self.spawn_rock(rock);
            loop {
                let rock_before_jetstream = starting_rock;
                let rock_after_jetstream = self.move_rock_by_jetstream(&rock_before_jetstream, jetstream);
                let rock_before_gravity = match self.is_out_of_bound(&rock_after_jetstream, min_height) || self.is_collided(&rock_after_jetstream) {
                    true => rock_before_jetstream,
                    false => rock_after_jetstream
                };
                // dbg!(rock_before_gravity.clone());
                let rock_after_gravity = self.move_rock_by_gravity(&rock_before_gravity);
                if self.is_collided(&rock_after_gravity) || self.is_out_of_bound(&rock_after_gravity, min_height) {
                    starting_rock = rock_before_gravity;
                    break;
                } else{
                    starting_rock = rock_after_gravity;
                }
                // dbg!(starting_rock.clone());
            }
            // dbg!(starting_rock.clone());
            self.map.extend(starting_rock.iter().map(|coord| (coord.0, coord.1)));
            // dbg!(self.map.clone());
            self.highest_height = max(self.highest_height, starting_rock.iter().map(|coord| coord.1 + 1).max().unwrap());
            
            let AMT = 4;
            if self.highest_height < AMT {
                index += 1;
                continue;
            }

            let jetstream_index = jetstream.get_index();
            let rock_index = rock_factory.get_index();

            let init_bit_mask = (1 << (7*AMT)) - 1;
            let mut starting_bit_mask = 1 << (7*AMT);

            for amt_to_shift in 0..AMT {
                let x_list = self.map.iter().filter(|(x,y)| *y == self.highest_height - amt_to_shift - 1).map(|(x,_)| x).collect::<Vec<&i64>>();
                let mul = 7 * amt_to_shift;
                for x in x_list {
                    let idk = mul + x;
                    starting_bit_mask |= 1 << idk;
                }
            }

            let final_bit_mask = starting_bit_mask & init_bit_mask;
            if memo.contains_key(&(rock_index, jetstream_index, final_bit_mask)) && index > jetstream.clone().positions.len() as i64 {
                let (before_height, before_index) = memo.get(&(rock_index, jetstream_index, final_bit_mask)).unwrap();
                let delta_height = self.highest_height - before_height;
                let delta_index = index - before_index;
                
                let cycle_to_bypass = (n - before_index) / delta_index;
                let remainder = (n - before_index) % delta_index;
                let height_on_cycle = cycle_to_bypass * delta_height + before_height;
                let height_after_cycle = lazy.get(&(before_index + remainder)).unwrap() - before_height;

                return height_on_cycle + height_after_cycle;
            } else {
                lazy.insert(index, self.highest_height);
                memo.insert((rock_index, jetstream_index, final_bit_mask), (self.highest_height, index));
            }
            index += 1;
        }

        return self.highest_height;
    }
}
    


fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    let mut jetstream = Jetstream::new(file.clone());
    let mut game = Game::new();
    println!("file length: {}", file.len());
    let result = game.move_n_number_of_rocks(&mut jetstream, 1000000000000);

    println!("result: {}", result);
}
