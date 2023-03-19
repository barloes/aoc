use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
};

use petgraph::{adj::NodeIndex, visit::EdgeRef, Graph, Undirected};

struct Node {
    name: String,
    profit: i32,
}

struct Game {
    map: Graph<Node, i32, Undirected>,
    visited: HashSet<NodeIndex>,
    // current_location: Option<NodeIndex>,
}

impl Game {
    fn new() -> Self {
        let map = Graph::<Node, i32, Undirected>::new_undirected();
        let visited = HashSet::new();
        Self { map, visited }
    }

    fn add_node(&mut self, node: Node) -> u32 {
        let node_index = self.map.add_node(node);
        node_index.index().try_into().unwrap()
    }

    fn add_path(&mut self, from: NodeIndex, to: NodeIndex) {
        self.map.add_edge(from.into(), to.into(), 1);
    }

    // change to precalculate
    fn get_path_with_max_profit(&mut self, n:i32, starting_node: &NodeIndex) -> (NodeIndex, i32, i32) {
        // slow option. iterate from current location to all other nodes -> List of Node, time_for_travel
        let mut queue: VecDeque<(NodeIndex, i32)> = VecDeque::<(NodeIndex, i32)>::new();
        let mut local_visited = HashMap::new();

        queue.push_back((*starting_node as NodeIndex, 0));
        let mut node_list = Vec::<(NodeIndex, i32)>::new();

        while !queue.is_empty() {
            let (current_node, time_on_travel) = queue.pop_front().unwrap();
            if local_visited.get(&current_node).is_some()
                && time_on_travel >= *local_visited.get(&current_node).unwrap()
            {
                continue;
            }
            node_list.push((current_node, time_on_travel));
            local_visited.insert(current_node, time_on_travel);
            for edge in self.map.edges(current_node.into()) {
                let next_node = edge.target().index().try_into().unwrap();
                queue.push_back((next_node, time_on_travel + 1));
            }
        }

        println!("node_list: {:?}", node_list);
        let mut mx_profit = 0;
        let mut mx_node = node_list[0].0;
        let mut fastest_time_on_travel = 0;
        for (node, time_on_travel) in node_list {
            if self.visited.get(&node).is_some() {
                continue;
            }
            let profit = get_profits(
                n,
                time_on_travel,
                self.map.node_weight(node.into()).unwrap().profit,
            );
            if profit > mx_profit {
                mx_profit = profit;
                mx_node = node;
                fastest_time_on_travel = time_on_travel;
            }
        }
        if mx_profit < 0 {
            return (mx_node, n, 0);
        }
        self.visited.insert(mx_node);
        return (mx_node, fastest_time_on_travel, mx_profit);
    }
}

// 
/*
load file
from path, profit, list of to path

each node should have information (distance to travel) to all nodes
precalulated from input file
eg. 
2 map ? to hold this information
A -> (B, 1), (C, 2) HashMap<String, HashMap<String, i32>> 
A -> Profit HashMap<String, i32>

using formula for calculating profit (n - d - 1) * Xi where (n-d - 1) is the total time left and Xi and the profit from opening the path
aim: get max profit from traversing all possible paths

recursive formula to get max profit
 - base case: highest profit from unvisited paths < 0. return 0
 - general case: highest profit from visited paths > 0
    - profit + fn(path without highest profit path, visited path without highest profit path, n - time_on_travel - 1)
 */
#[derive(Debug)]
struct Game2 {
    valve_to_profit_mapper: HashMap<String, i32>, 
    valve_to_valve_distance_mapper: HashMap<String, HashMap<String, i32>>,
    neighbour: HashMap<String, Vec<String>>, // only use to precalculate ^
}

impl Game2 {
    fn new() -> Self {
        let name_to_profit_mapper = HashMap::new();
        let name_to_name_to_distance_mapper = HashMap::new();
        Self {
            valve_to_profit_mapper: name_to_profit_mapper,
            valve_to_valve_distance_mapper: name_to_name_to_distance_mapper,
            neighbour: HashMap::new(),
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
            let mut visited = HashMap::<String, i32>::new();
            queue.push_back((from_valve.clone(), 0));

            while let Some((current_valve, distance)) = queue.pop_front() {
                if (visited.get(&current_valve).is_some() && distance >= *visited.get(&current_valve).unwrap()) {
                    continue;
                }
                visited.insert(current_valve.clone(), distance);

                if self.valve_to_valve_distance_mapper.get(from_valve).is_none() {
                    self.valve_to_valve_distance_mapper.insert(from_valve.clone(), HashMap::new());
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
    let game = load_input();

    let mut visited = HashSet::<String>::new();
    visited.insert("AA".to_string());
    let solution_1_ans = calculate_max_profit(&game, visited, "AA".to_string(), 30);
    println!("solution 1 ans: {:?}", solution_1_ans);

}

fn load_input() -> (Game2) {
    let mut game = Game2::new();
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    let mut node_to_node_index_map = HashMap::<String, NodeIndex>::new();
    let mut edge_list = Vec::<(String, String)>::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();
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

        game.add_valve_info(from_valve, profit, to_valve_list)
    }

    game.preculculate_distance();

    game
}

fn calculate_max_profit(game_info: &Game2, visited: HashSet<String>, from_valve: String, time_left: i32) -> i32 {
    if time_left <= 0 {
        return 0;
    }
    let mut max_profit = 0;

    let to_valve_list = game_info.valve_to_valve_distance_mapper.get(&from_valve).unwrap().iter()
    .filter(|&(k,v)| {
        let profit = *game_info.valve_to_profit_mapper.get(k).unwrap();
        let time_on_travel = *v;

        return get_profits(time_left, time_on_travel, profit) > 0 && !visited.contains(k)})
    .map(|(k,v)| k.to_string())
    .collect::<Vec<String>>();

    for to_valve in to_valve_list {
        let time_on_travel = *game_info.valve_to_valve_distance_mapper.get(&from_valve).unwrap().get(&to_valve).unwrap();
        let to_valve_profit = *game_info.valve_to_profit_mapper.get(&to_valve).unwrap();
        let updated_time_left = time_left - time_on_travel - 1;
        let mut updated_visited = visited.clone();
        updated_visited.insert(to_valve.clone());
        let temp_profit = get_profits(time_left, time_on_travel, to_valve_profit) + calculate_max_profit(game_info, updated_visited, to_valve, updated_time_left);            
        if temp_profit > max_profit {
            max_profit = temp_profit;
        }
    }
    
    max_profit
}

fn get_profits(n: i32, time_on_travel: i32, profit: i32) -> i32 {
    (n - time_on_travel - 1) * profit
}
