use std::{collections::{HashMap, VecDeque, HashSet}, fs::File, io::{BufReader, BufRead}, hash, clone, cmp::max};


struct Sensor {
    coord: (i32, i32),
    beacon: (i32, i32)
}

impl Sensor {
    fn new(coord: (i32, i32), beacon: (i32, i32)) -> Sensor {
        Sensor {
            coord: coord,
            beacon: beacon,
        }
    }
    
    fn get_dist(&self) -> i32 {
        (self.coord.0 - self.beacon.0).abs() + (self.coord.1 - self.beacon.1).abs()
    }

    fn optimised_run(&self, y:i32) -> HashSet<(i32,i32)> {
        let mut visited = HashSet::<(i32,i32)>::new();
        let dist = self.get_dist() - (self.coord.1 - y).abs();
        let start = self.coord.0 - dist;
        let end = self.coord.0 + dist;
        for i in start..=end {
            visited.insert((i, y));
        }
        // println!("{:?}", visited);

        visited
    }

    fn get_range(&self, y:i32) -> Option<(i32, i32)> {
        let dist = self.get_dist() - (self.coord.1 - y).abs();
        if dist < 0 {
            return None;
        }
        let start = self.coord.0 - dist;
        let end = self.coord.0 + dist;
        Some((start, end))
    }
}

struct Game {
    sensor_list: Vec<Sensor>,
    game_map: HashMap<(i32,i32), i32>,
}

impl Game {
    fn new(sensor_list: Vec<Sensor>) -> Game {
        let mut game_map = HashMap::<(i32,i32), i32>::new();
        for sensor in &sensor_list {
            game_map.insert((sensor.beacon.0, sensor.beacon.1), 2);
            game_map.insert((sensor.coord.0, sensor.coord.1), 3);
        }

        Game {
            sensor_list: sensor_list,
            game_map: game_map,
        }
    }

    fn run_v1(&mut self, y:i32) -> i32 {
        let mut hashset = HashSet::<(i32)>::new();
        for sensor in &self.sensor_list {
            if (y <=sensor.coord.1 + sensor.get_dist() && y >= sensor.coord.1 - sensor.get_dist()) {
                // println!("{} {} {}", sensor.coord.0, sensor.coord.1, sensor.get_dist());
                // self.print_map();
                // println!("{:?}, {:?}", sensor.coord, sensor.beacon);
                let temp = sensor.optimised_run(y);
                // println!("temp {:?}", temp);
                for (i,j) in temp {
                    if self.game_map.contains_key(&(i,j)) {
                        continue;
                    }
                    self.game_map.insert((i,j), 1);
                    if j == y {
                        hashset.insert(i);
                    }
                }
            }
        }
        hashset.len() as i32
    }

    fn run_v2(&mut self) {
        let start = 0;
        let bound = 4000000;

        for i in start..=bound {
            let mut sorted_range = Vec::<(i32, i32)>::new();
            for sensor in &self.sensor_list {
                if let Some(range) = sensor.get_range(i) {
                    sorted_range.push(range);
                }
            }   
            sorted_range.sort();
            println!("i:{}", i);
            // println!("{:?}", sorted_range);  
            let mut sorted_range_iter = sorted_range.iter();
            let mut cur = sorted_range_iter.next().unwrap().to_owned();
            while let Some(next_range) = sorted_range_iter.next() {
                // println!("i:{} {:?}, {:?}", i, cur, next_range);
                if cur.1 + 1 >= next_range.0 {
                    cur = (cur.0, max(cur.1, next_range.1));
                } else {
                    let x = cur.1 + 1;
                    let y = i;
                    get_solution_2(x, y);
                    return;
                }
            }        
        }

        panic!("no solution found");
    }

    fn print_map(&self, y: i32) {
        for x in -4 ..27 {
                if self.game_map.contains_key(&(x,y)) {
                    match self.game_map.get(&(x,y)) {
                        Some(1) => print!("1"),
                        Some(2) => print!("2"),
                        Some(3) => print!("3"),
                        _ => print!("-"),
                    }       
                } else {
                    print!("0");
                }
        }
        println!()
    }
}

fn main() {
    let sensor_list = load_sensor_list();
    let mut game = Game::new(sensor_list);
    game.run_v2();
}

fn load_sensor_list() -> Vec<Sensor> {
    let mut sensor_list = Vec::<Sensor>::new();

    let file = File::open("input").unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let splitted = line.split(" ").collect::<Vec<&str>>();

        let pattern = ['x', 'y', ':', ',', '='];
        let sensor_x = splitted[2].replace(pattern, "").parse::<i32>().unwrap();
        let sensor_y = splitted[3].replace(pattern, "").parse::<i32>().unwrap();

        let beacon_x = splitted[8].replace(pattern, "").parse::<i32>().unwrap();
        let beacon_y = splitted[9].replace(pattern, "").parse::<i32>().unwrap();
        let sensor = Sensor::new((sensor_x, sensor_y), (beacon_x, beacon_y));
        sensor_list.push(sensor);
    }

    sensor_list
}

fn get_solution_2(x: i32, y:i32) {
    let ans: i128 = x as i128 * 4000000 as i128 + y as i128;
    println!("solution 2: {}", ans);
}