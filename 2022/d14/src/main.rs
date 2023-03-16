use std::{
    borrow::BorrowMut,
    cmp::{max, min},
    collections::HashMap,
    hash::Hash,
    iter::Map,
};

// sand = 0
// rock = #
// none = .
#[derive(PartialEq, Debug)]
enum GameObject {
    Sand,
    Rock,
    None,
}

struct Sand {
    x: i32,
    y: i32,
}

impl Sand {
    fn new(x: i32, y: i32) -> Sand {
        Sand { x, y }
    }

    fn get_final_pos(
        &mut self,
        block_map: &HashMap<(i32, i32), GameObject>,
        game: &Game,
    ) -> (bool, i32, i32) {
        loop {
            if game.is_out_of_bound(self.x, self.y) {
                return (false, self.x, self.y);
            }

            if self.can_move(self.move_down(), block_map) {
                (self.x, self.y) = self.move_down();
            } else if self.can_move(self.move_bottom_left(), block_map) {
                (self.x, self.y) = self.move_bottom_left();
            } else if self.can_move(self.move_bottom_right(), block_map) {
                (self.x, self.y) = self.move_bottom_right();
            } else {
                break;
            }
        }

        (true, self.x, self.y)
    }

    fn get_final_pos_v2(
        &mut self,
        block_map: &HashMap<(i32, i32), GameObject>,
        game: &Game,
    ) -> (bool, i32, i32) {
        loop {
            if self.can_move(self.move_down(), block_map) {
                (self.x, self.y) = self.move_down();
            } else if self.can_move(self.move_bottom_right(), block_map) {
                (self.x, self.y) = self.move_bottom_right();
            } else if self.can_move(self.move_bottom_left(), block_map) {
                (self.x, self.y) = self.move_bottom_left();
            } else {
                break;
            }

            if self.reach_fixed_wall((self.x, self.y), game.y_coord.1) {
                break;
            }

            // println!("x: {}, y: {}, max_y: {}, reached: {}", self.x, self.y, game.y_coord.1, self.reach_fixed_wall((self.x, self.y), game.y_coord.1));
        }

        (true, self.x, self.y)
    }

    fn reach_fixed_wall(&self, coord: (i32, i32), max_y: i32) -> bool {
        // println!("coord: {:?} max:{}", coord, max_y);
        if coord.1 >= max_y + 1 {
            return true;
        }

        false
    }

    fn can_move(&self, coord: (i32, i32), block_map: &HashMap<(i32, i32), GameObject>) -> bool {
        if block_map.contains_key(&coord) {
            return false;
        }
        true
    }

    fn move_bottom_left(&self) -> (i32, i32) {
        (self.x - 1, self.y + 1)
    }

    fn move_bottom_right(&self) -> (i32, i32) {
        (self.x + 1, self.y + 1)
    }

    fn move_down(&self) -> (i32, i32) {
        (self.x, self.y + 1)
    }
}

struct InitBlockResponse {
    block_list: Vec<(i32, i32)>,
    x_coord: (i32, i32),
    y_coord: (i32, i32),
}

// 2d map
struct Game {
    block_map: HashMap<(i32, i32), GameObject>,
    x_coord: (i32, i32),
    y_coord: (i32, i32),
}

// sand goes all the way down
// if no path, go diagonal left (down + left)
// else go diagonal right (down + right)
// take note! the higher the y value, the lower the position

impl Game {
    fn new(block_list: Vec<String>) -> Game {
        let init_block_response = initialise_blocks(block_list);
        let block_list_to_add = init_block_response.block_list;
        let x_coord = init_block_response.x_coord;
        let y_coord = init_block_response.y_coord;
        // initialise map with block_list
        let mut block_map: HashMap<(i32, i32), GameObject> = HashMap::new();
        for block in block_list_to_add {
            block_map.insert(block, GameObject::Rock);
        }
        // block_map.insert((500,0), GameObject::Sand);
        // initialise highest_pos
        let mut highest_pos = HashMap::new();
        for x in x_coord.0..=x_coord.1 {
            let mut hi = 0;
            for y in (y_coord.0..=y_coord.1) {
                if block_map.contains_key(&(x, y)) && block_map[&(x, y)] == GameObject::Rock {
                    break;
                }
                hi = y;
            }
            highest_pos.insert(x, hi);
        }
        Game {
            block_map,
            x_coord,
            y_coord,
        }
    }

    fn add_sand(&mut self, x: i32) -> bool {
        let mut nxt_x = x.clone();
        let mut nxt_y = 0;
        // println!("land on x: {}, y: {}", nxt_x, nxt_y);

        let mut sand = Sand::new(nxt_x, nxt_y);
        let (is_poss, nxt_x, nxt_y) = sand.get_final_pos(&self.block_map, &self);
        if !is_poss {
            return false;
        }

        self.block_map.insert((nxt_x, nxt_y), GameObject::Sand);
        return true;
    }

    fn add_sand_v2(&mut self, x: i32) -> bool {
        let mut nxt_x = x.clone();
        let mut nxt_y = 0;
        // println!("land on x: {}, y: {}", nxt_x, nxt_y);
        if self.block_map.contains_key(&(x, 0)) {
            return false;
        }
        let mut sand = Sand::new(nxt_x, nxt_y);
        let (is_poss, nxt_x, nxt_y) = sand.get_final_pos_v2(&self.block_map, &self);
        if !is_poss {
            return false;
        }

        self.block_map.insert((nxt_x, nxt_y), GameObject::Sand);
        return true;
    }

    fn is_out_of_bound(&self, x: i32, y: i32) -> bool {
        if x < self.x_coord.0 || x > self.x_coord.1 || y < self.y_coord.0 || y > self.y_coord.1 {
            return true;
        }
        false
    }

    fn print_map(&self) {
        for y in (self.y_coord.0..=self.y_coord.1) {
            for x in self.x_coord.0..=self.x_coord.1 {
                match self.block_map.get(&(x, y)) {
                    Some(GameObject::Sand) => print!("s"),
                    Some(GameObject::Rock) => print!("#"),
                    Some(GameObject::None) => print!("."),
                    None => print!("."),
                }
            }
            println!();
        }
    }

    fn print_map_v2(&self) {
        for y in (self.y_coord.0..=self.y_coord.1 + 1) {
            for x in self.x_coord.0..=self.x_coord.1 {
                match self.block_map.get(&(x, y)) {
                    Some(GameObject::Sand) => print!("s"),
                    Some(GameObject::Rock) => print!("#"),
                    Some(GameObject::None) => print!("."),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("input").unwrap();

    let block_list = file.lines().map(|x| x.to_string()).collect::<Vec<String>>();
    let mut game = Game::new(block_list);

    let mut counter = 0;
    loop {
        // game.print_map();
        match game.add_sand(500) {
            true => {
                counter += 1;
                continue;
            }
            false => break,
        }
    }
    println!("solution 1 ans: {}", counter);
}

fn initialise_blocks(block_list: Vec<String>) -> InitBlockResponse {
    let mut x_coord = (500, 500);
    let mut y_coord = (0, 0);

    let mut block_list_to_return = Vec::new();
    for block_str in block_list {
        let mut starting_block: Option<(i32, i32)> = None;
        block_str.split("->").for_each(|block| {
            let block = block.trim();
            // convert string to i32
            let coord = block
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if starting_block != None {
                for x in min(starting_block.unwrap().0, coord[0])
                    ..=max(starting_block.unwrap().0, coord[0])
                {
                    block_list_to_return.push(((x, coord[1])));
                }
                for y in min(starting_block.unwrap().1, coord[1])
                    ..=max(starting_block.unwrap().1, coord[1])
                {
                    block_list_to_return.push(((coord[0], y)));
                }
            }

            block_list_to_return.push((coord[0], coord[1]));
            starting_block = Some((coord[0], coord[1]));
        });
    }

    for item in block_list_to_return.iter() {
        x_coord.0 = min(x_coord.0, item.0);
        x_coord.1 = max(x_coord.1, item.0);
        y_coord.0 = min(y_coord.0, item.1);
        y_coord.1 = max(y_coord.1, item.1);
    }

    InitBlockResponse {
        block_list: block_list_to_return,
        x_coord,
        y_coord,
    }
}
