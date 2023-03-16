use std::{fs::File, io::BufReader, io::BufRead, collections::{VecDeque, HashSet, HashMap}, process::exit};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
    steps: i32,
}

impl Position {
    fn new(x: i32, y: i32, steps: i32) -> Position {
        Position {
            x,
            y,
            steps,
        }
    }

    fn get_neighbours(&self, map: &Vec<String>) -> Vec<(i32, i32)> {
        let mut neighbours = Vec::<(i32, i32)>::new();
        let n = map.len();
        let m = map[0].len();

        let mut current_elevation = map[self.x as usize].chars().nth(self.y as usize).unwrap();
        current_elevation = update_elavation(current_elevation);

        let dx = [0, 0, 1, -1];
        let dy = [1, -1, 0, 0];
        for i in 0..4 {
            let new_x = self.x + dx[i];
            let new_y = self.y + dy[i];
            if new_x >= 0 && new_x < n as i32 && new_y >= 0 && new_y < m as i32 {
                let mut path_elavation = map[new_x as usize].chars().nth(new_y as usize).unwrap();
                
                path_elavation = update_elavation(path_elavation);
                if path_elavation as u8 <= current_elevation as u8 + 1 {
                    neighbours.push((new_x, new_y));
                } 
            }
        }
        println!("neighbours: {:?}", neighbours);
        return neighbours;
    }

    fn get_path_from(&self, map: &Vec<String>) -> Vec<(i32, i32)> {
        let mut path = Vec::<(i32, i32)>::new();
        let n = map.len();
        let m = map[0].len();

        let mut current_elevation = map[self.x as usize].chars().nth(self.y as usize).unwrap();
        current_elevation = update_elavation(current_elevation);

        let dx = [0, 0, 1, -1];
        let dy = [1, -1, 0, 0];
        for i in 0..4 {
            let new_x = self.x + dx[i];
            let new_y = self.y + dy[i];
            if new_x >= 0 && new_x < n as i32 && new_y >= 0 && new_y < m as i32 {
                let mut path_elavation = map[new_x as usize].chars().nth(new_y as usize).unwrap();
                
                path_elavation = update_elavation(path_elavation);

                if path_elavation as u8 >= current_elevation as u8 -1 {
                    path.push((new_x, new_y));
                } 
            }
        }
        println!("neighbours: {:?}", path);
        return path;
    }

}
    

fn main() {
    let file = File::open("input").expect("file not found");
    let reader = BufReader::new(file);

    let mut map = Vec::<String>::new();
    for line_to_unwrap in reader.lines(){
        let line = line_to_unwrap.unwrap();
        map.push(line);
    }

    // let result = solution_1(&map);
    // println!("{}", result);

    let result = solution_2(&map);
    println!("{}", result);

}

// solution 1
fn solution_1(map: &Vec<String>) -> i32 {
    let n = map.len();
    let m = map[0].len();

    let (starting_pos_x, starting_pos_y, ending_pos_x, ending_pos_y) = find_positions(&map, n, m);
    let mut queue = VecDeque::<(Position)>::new();
    queue.push_back(Position::new(starting_pos_x, starting_pos_y, 0));
    let mut visited = HashMap::<(i32, i32),(i32)>::new();
    visited.insert((starting_pos_x, starting_pos_y), 0);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        println!("current: {:?}", current);
        for neighbour in current.get_neighbours(&map) {
            let neighbour_elavation = map[neighbour.0 as usize].chars().nth(neighbour.1 as usize).unwrap();
            if neighbour_elavation == 'E' {
                return current.steps + 1;
            }
            
            // if path found have a took a lesser step to reached. dont go there anymore.
            if let Some(steps) = visited.get(&neighbour) {
                if *steps <= current.steps + 1 {
                    continue;
                }
            }

            let new_pos = Position::new(neighbour.0, neighbour.1, current.steps + 1);
            queue.push_back(new_pos);
            visited.insert((neighbour.0, neighbour.1), current.steps + 1);
        }
    }
    panic!("No path found");
}

fn solution_2(map: &Vec<String>) -> i32 {
    let n = map.len();
    let m = map[0].len();

    let (starting_pos_x, starting_pos_y, ending_pos_x, ending_pos_y) = find_positions(&map, n, m);
    let mut queue = VecDeque::<(Position)>::new();
    queue.push_back(Position::new(ending_pos_x, ending_pos_y, 0));
    let mut visited = HashMap::<(i32, i32),(i32)>::new();
    visited.insert((ending_pos_x, ending_pos_y), 0);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        println!("current: {:?}", current);
        for neighbour in current.get_path_from(&map) {
            let mut neighbour_elavation = map[neighbour.0 as usize].chars().nth(neighbour.1 as usize).unwrap();
            neighbour_elavation = update_elavation(neighbour_elavation);
            if neighbour_elavation == 'a' {
                return current.steps + 1;
            }
            
            // if path found have a took a lesser step to reached. dont go there anymore.
            if let Some(steps) = visited.get(&(neighbour.0, neighbour.1)) {
                if *steps <= current.steps + 1 {
                    continue;
                }
            }

            let new_pos = Position::new(neighbour.0, neighbour.1, current.steps + 1);
            queue.push_back(new_pos);
            visited.insert((neighbour.0, neighbour.1), current.steps + 1);
        }
    }
    panic!("No path found");
}

fn find_positions(map: &Vec<String>, n: usize, m: usize) -> (i32,i32,i32,i32) {
    let mut starting_pos = (-1, -1);
    let mut ending_pos = (-1, -1);

    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                starting_pos = (i as i32, j as i32);
            }
            if c =='E' {
                ending_pos = (i as i32, j as i32);
            }
        }
    }
    
    return (starting_pos.0 , starting_pos.1 , ending_pos.0 , ending_pos.1);
}


fn update_elavation(elavation: char)->char {
    if elavation == 'S'{
        return 'a';
    } else if elavation == 'E' {
        return 'z';
    }
    else {
        return elavation;
    }
}