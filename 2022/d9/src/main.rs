use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Unit {
    x: i32,
    y: i32,
}

struct Game {
    head: Unit,
    tail: Unit,
    visited_path: HashSet<(i32, i32)>,
}

struct Game_v2 {
    units: Vec<Unit>,
    visited_path: HashSet<(i32, i32)>,
}

impl Game {
    fn new() -> Game {
        Game {
            head: Unit { x: 0, y: 0 },
            tail: Unit { x: 0, y: 0 },
            visited_path: HashSet::new(),
        }
    }

    fn move_head(&mut self, direction: char, distance: i32) {
        for i in 0..distance {
            match direction {
                'U' => self.head.y += 1,
                'D' => self.head.y -= 1,
                'L' => self.head.x += 1,
                'R' => self.head.x -= 1,
                _ => println!("Invalid direction"),
            }

            // move tail if head and tail are not touching
            if !self.is_head_and_tail_touching(self.tail.x, self.tail.y) {
                self.move_tail();
            }

            // println!("moving head");
            // println!("head x: {}, y: {}", self.head.x, self.head.y);
            // println!("tail x: {}, y: {}", self.tail.x, self.tail.y);
            self.visited_path.insert((self.tail.x, self.tail.y));
        }
    }

    fn is_head_and_tail_touching(&self, tail_x: i32, tail_y: i32) -> bool {
        let x_dist = (self.head.x - tail_x).abs();
        let y_dist = (self.head.y - tail_y).abs();

        if x_dist <= 1 && y_dist <= 1 {
            return true;
        } else {
            return false;
        }
    }

    fn move_tail(&mut self) {
        let (head_x, head_y) = (self.head.x, self.head.y);
        let (tail_x, tail_y) = (self.tail.x, self.tail.y);

        // println!("moving tail");
        // println!("head x: {}, y: {}", head_x, head_y);
        // if tail same row and col , move l,r,u,d
        if self.tail.x == self.head.x || self.tail.y == self.head.y {
            let dx = [0, 0, 1, -1];
            let dy = [1, -1, 0, 0];
            for i in 0..4 {
                // println!("tail x: {}, y: {}", tail_x, tail_y);
                let (new_x, new_y) = (tail_x + dx[i], tail_y + dy[i]);
                if self.is_head_and_tail_touching(new_x, new_y) {
                    self.tail.x = new_x;
                    self.tail.y = new_y;
                    return;
                }
            }
            panic!("Tail cannot move in same row or col")
        } else {
            // else move diagonally
            let dx = [1, 1, -1, -1];
            let dy = [1, -1, 1, -1];
            for i in 0..4 {
                let (new_tail_x, new_tail_y) = (tail_x + dx[i], tail_y + dy[i]);
                if self.is_head_and_tail_touching(new_tail_x, new_tail_y) {
                    self.tail.x = new_tail_x;
                    self.tail.y = new_tail_y;
                    return;
                }
            }
            panic!("Tail cannot move diagonally")
        }
    }

    fn get_visited_path(&self) -> &HashSet<(i32, i32)> {
        &self.visited_path
    }
}

impl Game_v2 {
    fn new(number_of_unit: usize) -> Game_v2 {
        Game_v2 {
            units: vec![Unit { x: 0, y: 0 }; number_of_unit],
            visited_path: HashSet::new(),
        }
    }
    fn move_head(&mut self, direction: char, distance: i32) {
        for i in 0..distance {
            match direction {
                'U' => self.units[0].y += 1,
                'D' => self.units[0].y -= 1,
                'L' => self.units[0].x += 1,
                'R' => self.units[0].x -= 1,
                _ => println!("Invalid direction"),
            }

            for i in 1..self.units.len() {
                if !self.is_head_and_tail_touching(
                    self.units[i - 1].x,
                    self.units[i - 1].y,
                    self.units[i].x,
                    self.units[i].y,
                ) {
                    let head_index = i - 1;
                    let tail_inedx = i;

                    self.move_tail(head_index,tail_inedx);
                }
            }

            // move tail if head and tail are not touching

            // println!("moving head");
            // println!("head x: {}, y: {}", self.head.x, self.head.y);
            // println!("tail x: {}, y: {}", self.tail.x, self.tail.y);
            let n = self.units.len() - 1; 
            self.visited_path.insert((self.units[n].x, self.units[n].y));
        }
    }

    fn is_head_and_tail_touching(
        &self,
        head_x: i32,
        head_y: i32,
        tail_x: i32,
        tail_y: i32,
    ) -> bool {
        let x_dist = (head_x - tail_x).abs();
        let y_dist = (head_y - tail_y).abs();

        if x_dist <= 1 && y_dist <= 1 {
            return true;
        } else {
            return false;
        }
    }

    fn move_tail(&mut self, head_index: usize, tail_index: usize) {
        let (head_x, head_y) = (self.units[head_index].x, self.units[head_index].y);
        let (tail_x, tail_y) = (self.units[tail_index].x, self.units[tail_index].y);

        // println!("moving tail");
        // println!("head x: {}, y: {}", head_x, head_y);
        // if tail same row and col , move l,r,u,d
        if head_x == tail_x || head_y == tail_y{
            let dx = [0, 0, 1, -1];
            let dy = [1, -1, 0, 0];
            for i in 0..4 {
                // println!("tail x: {}, y: {}", tail_x, tail_y);
                let (new_tail_x, new_tail_y) = (tail_x + dx[i], tail_y + dy[i]);
                if self.is_head_and_tail_touching(head_x, head_y, new_tail_x, new_tail_y) {
                    self.units[tail_index].x = new_tail_x;
                    self.units[tail_index].y = new_tail_y;
                    return;
                }
            }
            panic!("Tail cannot move in same row or col")
        } else {
            // else move diagonally
            let dx = [1, 1, -1, -1];
            let dy = [1, -1, 1, -1];
            for i in 0..4 {
                let (new_tail_x, new_tail_y) = (tail_x + dx[i], tail_y + dy[i]);
                if self.is_head_and_tail_touching(head_x, head_y, new_tail_x, new_tail_y) {
                    self.units[tail_index].x = new_tail_x;
                    self.units[tail_index].y = new_tail_y;
                    return;
                }
            }
            panic!("Tail cannot move diagonally")
        }
    }

    fn get_visited_path(&self) -> &HashSet<(i32, i32)> {
        &self.visited_path
    }
}

fn main() {
    // solution_1();
    solution_2()
}

fn solution_1() {
    let file = std::fs::read_to_string("test").unwrap();
    let mut game = Game_v2::new(2);
    for line in file.lines() {
        let mut iter = line.split_whitespace();
        let direction = iter.next().unwrap().chars().next().unwrap();
        let distance = iter.next().unwrap().parse::<i32>().unwrap();
        game.move_head(direction, distance);
    }

    println!("visited path length: {}", game.get_visited_path().len());
}

fn solution_2() {
    let file = std::fs::read_to_string("input").unwrap();
    let mut game = Game_v2::new(10);
    for line in file.lines() {
        let mut iter = line.split_whitespace();
        let direction = iter.next().unwrap().chars().next().unwrap();
        let distance = iter.next().unwrap().parse::<i32>().unwrap();
        game.move_head(direction, distance);
    }

    println!("visited path length: {}", game.get_visited_path().len());
}
