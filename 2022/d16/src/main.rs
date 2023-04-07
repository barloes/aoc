use core::time;
use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    fs::File,
    io::Read, f32::INFINITY, path, cmp::max,
};

// declare global inf 
const INF: i64 = 1 << 30;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Copy, Ord, PartialOrd)]
struct Bitmask {
    mask: i64,
    size: i32,
}

struct TunnelMap {
    name_to_index_mapper: BTreeMap<String, i32>,
    index_to_name_mapper: BTreeMap<i32, String>,
    // name_to_profit_mapper: HashMap<String, i32>,
    index_to_profit_mapper: BTreeMap<i32, i32>,
    tunnel_map: Vec<Vec<i64>>,
    highest_index: i32, // use for generating index for new valve
}

impl TunnelMap {
    fn new(size: i32) -> Self {
        // initialise with map with size
        let mut tunnel_map = vec![vec![INF as i64; size as usize]; size as usize];
        let highest_index = 0;
        Self {
            name_to_index_mapper: BTreeMap::new(),
            index_to_name_mapper: BTreeMap::new(),
            index_to_profit_mapper: BTreeMap::new(),
            tunnel_map: tunnel_map,
            highest_index: highest_index,
        }
    }

    fn get_index_list(&self) -> Vec<usize> {
        self.index_to_profit_mapper.iter()
            .map(|(k, _)| *k as usize).collect()
    }

    fn preculculate_distance(&mut self) {
        let size = self.tunnel_map.len();

        for k in 0..size {
            for i in 0..size {
                for j in 0..size {
                    if self.tunnel_map[i][j] > self.tunnel_map[i][k] + self.tunnel_map[k][j] {
                        self.tunnel_map[i][j] = self.tunnel_map[i][k] + self.tunnel_map[k][j];
                    }
                }
            }
        }
    }

    fn print_distance(&self) {
        for i in 0..self.tunnel_map.len() {
            for j in 0..self.tunnel_map.len() {
                print!("{} ", self.tunnel_map[i][j]);
            }
            println!();
        }
    }

    fn get_profit(&self, index: usize) -> i32 {
        *self.index_to_profit_mapper.get(&(index as i32)).unwrap()
    }

    fn get_time_on_travel(&self, from: Option<i32>, to: usize) -> i32 {
        
        if let Some(from) = from {
            self.tunnel_map[from as usize][to] as i32
        } else {
            0
        }
    }

    fn get_neighbour(&self, index:i32) -> Vec<usize> {
        self.tunnel_map.get(index as usize).unwrap()
            .iter()
            .enumerate()
            .filter(|(k, v)| **v != (INF as i64) && k != &(index as usize) && self.index_to_profit_mapper.get(&(*k as i32)).unwrap() > &0)
            .map(|(index, _)| index).collect()
    }

    fn add_valve(&mut self, name: String, profit:i32, neighbour: Vec<String>) {
        let current_valve_index = self.get_valve_index(name, Some(profit));
        for neighbour in neighbour {
            let neighbour_index = self.get_valve_index(neighbour, None);
            self.tunnel_map[current_valve_index as usize][neighbour_index as usize] = 1;
        }
    }

    fn get_valve_index(&mut self, name: String, profit: Option<i32>) -> i32 {
        if self.name_to_index_mapper.get(&name).is_none() {
            self.name_to_index_mapper.insert(name.clone(), self.highest_index);
            self.index_to_name_mapper.insert(self.highest_index, name.clone());
 
            self.highest_index += 1;
        }

        let index_to_return = *self.name_to_index_mapper.get(&name).unwrap();
        if let Some(profit) = profit {
            self.index_to_profit_mapper.insert(index_to_return, profit);
        }
        index_to_return
    }
    
}

impl Bitmask{
    fn new(size: i32) -> Self {
        Self {
            mask: 1<<size,
            size,
        }
    }

    fn set(&mut self, index: i32) {
        self.mask |= 1 << index;
    }

    fn get(&self, index: i32) -> bool {
        (self.mask & 1<<index) != 0
    }

    fn get_opposite(&self) -> Bitmask {
        Bitmask {
            mask: self.mask ^ (1<<self.size - 1),
            size: self.size,
        }
    }
}

#[derive(Debug)]
struct Game2 {
    valve_to_profit_mapper: BTreeMap<String, i32>, 
    valve_to_valve_distance_mapper: BTreeMap<String, BTreeMap<String, i32>>,
    neighbour: BTreeMap<String, Vec<String>>, // only use to precalculate ^
}

impl Game2 {
    fn new() -> Self {
        let name_to_profit_mapper = BTreeMap::new();
        let name_to_name_to_distance_mapper = BTreeMap::new();
        Self {
            valve_to_profit_mapper: name_to_profit_mapper,
            valve_to_valve_distance_mapper: name_to_name_to_distance_mapper,
            neighbour: BTreeMap::new(),
        }
    }

    fn add_valve_info(&mut self, from_valve: String, profit: i32, to_valve_list: Vec<String>) {
        self.valve_to_profit_mapper.insert(from_valve.clone(), profit);
        self.neighbour.insert(from_valve.clone(), to_valve_list);
    }

    fn preculculate_distance(&mut self) {
        let valve_list = self.valve_to_profit_mapper.keys().collect::<Vec<&String>>();
        // get all possible paths from current location and update the distance in valve_to_valve_distance_mapper
        for from_valve in valve_list {
            // (to_valve, distance)
            let mut queue = VecDeque::<(String, i32)>::new(); 
            let mut visited = BTreeMap::<String, i32>::new();
            queue.push_back((from_valve.clone(), 0));

            while let Some((current_valve, distance)) = queue.pop_front() {
                if (visited.get(&current_valve).is_some() && distance >= *visited.get(&current_valve).unwrap()) {
                    continue;
                }
                visited.insert(current_valve.clone(), distance);

                if self.valve_to_valve_distance_mapper.get(from_valve).is_none() {
                    self.valve_to_valve_distance_mapper.insert(from_valve.clone(), BTreeMap::new());
                }
                if current_valve != *from_valve {
                    self.valve_to_valve_distance_mapper.get_mut(from_valve).unwrap().insert(current_valve.clone(), distance);
                }
                for neighbour in self.neighbour.get(&current_valve).unwrap() {
                    queue.push_back((neighbour.clone(), distance + 1));
                }
            }
        }
    }
    
}

fn main() {
    let mut game = load_input();
    // game.print_distance();


    let visiting_index = game.get_valve_index("AA".to_string(), None);
    let mut visited = Bitmask::new(game.highest_index);
    visited.set(visiting_index);
    let mut memo = BTreeMap::<(Bitmask), i32>::new();
    let (solution_1_ans) = calculate_max_profit_2(&game, Some(visiting_index), &visited, &mut memo, 30, 0);
    println!("solution 1 memo: {:?}", memo);
    println!("solution 1 ans: {:?}", solution_1_ans);


    let mut solution2_memo1 = BTreeMap::<(Bitmask), i32>::new();

    let to_visit_list = game.get_index_list();
    
    // path you opened
    let mut new_visited = Bitmask::new(game.highest_index);
    calculate_max_profit_2(&game, Some(visiting_index), &new_visited, &mut solution2_memo1, 26, 0);

    let initMask = Bitmask::new(game.highest_index).mask - 1;

    let mut ans = 0;
    println!("solution 2 memo: {:?}", solution2_memo1);
    println!("solution 2 memo len: {:?}", solution2_memo1.len());

    let idk = Bitmask::new(game.highest_index);
    for (key1, value1) in &solution2_memo1 {
        let mask_1 = key1.mask;
        for (key2, value2) in &solution2_memo1 {
            let mask_2 = key2.mask;

            if mask_1 & mask_2 & initMask == 0{
                // println!("{:b}\n{:b}\n{:b} {:b}\n", mask_1, mask_2, initMask, mask_1&mask_2&initMask);

                // println!("key1: {:?}, key2: {:?}", key1, key2);
                ans = max(ans, value1 + value2);
            } 
        }
    }
    println!("solution 2 ans: {:?}", ans);


}

fn load_input() -> (TunnelMap) {
    let mut game = Game2::new();

    let mut file = File::open("test").unwrap();
    let mut contents = String::new();
    let mut edge_list = Vec::<(String, String)>::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();
    let mut game_2 = TunnelMap::new(lines.clone().count() as i32);

    let pattern = ['r', 'a', 't', 'e', '=', ';', ','];
    for line in lines {
        let mut splitted: Vec<&str> = line.split_whitespace().collect();
        let from_valve = splitted.get(1).unwrap().to_string();
        let profit = splitted
            .get(4)
            .unwrap()
            .replace(&pattern[..], "")
            .parse::<i32>()
            .unwrap();
        // from 9 to end
        let mut to_valve_list: Vec<String> = splitted
            .clone()
            .drain(9..)
            .map(|e| e.to_string().replace(pattern, ""))
            .collect();

        // game.add_valve_info(from_valve, profit, to_valve_list);
        game_2.add_valve(from_valve, profit, to_valve_list);
    }

    // game.preculculate_distance();
    game_2.preculculate_distance();
    game_2
}

fn calculate_max_profit_2(game_info: &TunnelMap, visiting_index: Option<i32>,visited: &Bitmask, memo:&mut BTreeMap<(Bitmask), i32>, time_left: i32, total_profit:i32) -> (i32) {
    update_memo(memo, visited.clone(), total_profit);

    let mut max_profit = 0;
    
    let to_visit_list = match visiting_index {
        Some(index) => game_info.get_neighbour(index),
        None => game_info.get_index_list()
    };
        
    for to_visit in to_visit_list {
        if visited.get(to_visit as i32) {
            continue;
        }

        let mut updated_visited = visited.clone();

        let time_on_travel = game_info.get_time_on_travel(visiting_index, to_visit);
        let to_visit_profit = game_info.get_profit(to_visit);

        let updated_time_left = time_left - time_on_travel - 1;
        updated_visited.set(to_visit as i32);

        let current_profit = get_profits(time_left, time_on_travel, to_visit_profit);
        if current_profit <= 0 || time_left - time_on_travel - 1 <= 0 {
            continue;
        }
        let (after_profit) = calculate_max_profit_2(game_info, Some(to_visit as i32), &updated_visited.clone(), memo, updated_time_left, total_profit + current_profit);
        let temp_profit = current_profit + after_profit;
        if temp_profit > max_profit {
            max_profit = temp_profit;
        }
    }

    return (max_profit);
}

fn update_memo(memo: &mut BTreeMap<(Bitmask), i32>, visited: Bitmask, profit: i32) {
    let new_key = (visited);
    let current_value = memo.get(&new_key).unwrap_or(&0);
    memo.insert(new_key, max(*current_value, profit));
}


fn get_profits(n: i32, time_on_travel: i32, profit: i32) -> i32 {
    (n - time_on_travel - 1) * profit
}
