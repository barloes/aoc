use std::{f64::MIN, collections::{BTreeMap, BTreeSet}};

use petgraph::{graph::NodeIndex, visit::EdgeRef, Graph};

#[derive(Debug)]
struct Node {
    name: String,
    size: u32,
    is_directory: bool,
    parent_node: Option<NodeIndex>,
}

struct NodeGraph {
    graph: Graph<Node, ()>,
    root: NodeIndex,
    current_node: NodeIndex,
}

impl NodeGraph {
    fn new() -> Self {
        let mut graph = Graph::<Node, ()>::new();
        let root: NodeIndex = graph.add_node(Node {
            name: "/".to_string(),
            size: 0,
            is_directory: true,
            parent_node: None,
        });
        NodeGraph {
            graph,
            root,
            current_node: root,
        }
    }

    fn add_node(&mut self, name: String, isDirectory: bool, size: Option<i32>) {
        let node = Node {
            name: name,
            size: size.unwrap_or(0) as u32,
            is_directory: isDirectory,
            parent_node: Some(self.current_node),
        };
        let new_node_index = self.graph.add_node(node);
        self.graph.add_edge(self.current_node, new_node_index, ());
    }

    // fn add_edge(&mut self, parent: NodeIndex, child: NodeIndex) {
    //     self.graph.add_edge(parent, child, ());
    // }

    fn change_to_parent_node(&mut self) {
        let parent = self.graph[self.current_node].parent_node.unwrap();
        self.current_node = parent;
    }

    fn change_to_child_node(&mut self, wanted_name: String) {
        for edge in self.graph.edges(self.current_node) {
            let child_name = &self.graph[edge.target()].name;
            print!("comparing: child: {} wanted {} ", child_name, wanted_name);
            if *child_name == wanted_name {
                self.current_node = edge.target();
                return;
            };
        }
        panic!("Node not found")
    }

    fn get_root(&self) -> NodeIndex {
        self.root
    }

    fn get_graph(&self) -> &Graph<Node, ()> {
        &self.graph
    }

    fn print(&self) {
        println!("{:?}", self.graph);
    }
}

const FILE_NAME: &str = "input";
const SOLUTION_1_MAX_SIZE: u32 = 100000;
static mut SOLUTION_1_ANS: u32 = 0;
static mut SET: BTreeSet<u32> = BTreeSet::new();

fn main() {
    // read file
    let input = std::fs::read_to_string(FILE_NAME).unwrap();

    let mut graph = NodeGraph::new();
    for line in input.lines().skip(1) {
        println!("{}", line);
        let splitted = line.split_whitespace().collect::<Vec<&str>>();

        match splitted[0] {
            "$" => {
                // command either cd or ls
                match splitted[1] {
                    "cd" => {
                        let path = splitted[2].to_string();
                        if path == ".." {
                            graph.change_to_parent_node();
                        } else {
                            graph.change_to_child_node(path);
                        }
                    }
                    "ls" => {
                        // do nothing
                    }
                    _ => {
                        panic!()
                    }
                }
            }
            _ => {
                match splitted[0] {
                    // dir, dirname
                    "dir" => {
                        let dir_name = splitted[1].to_string();
                        graph.add_node(dir_name, true, None);
                    }
                    // size, filename
                    _ => {
                        let file_size = splitted[0].parse::<i32>().unwrap();
                        let file_name = splitted[1].to_string();
                        graph.add_node(file_name, false, Some(file_size));
                    }
                }
            }
        }
    }

    let root = graph.get_root();
    let total_size: i32 = get_all_directory_size(root, graph.get_graph())
        .try_into()
        .unwrap();
    // graph.print();

    unsafe {
        println!("solution 1: {}", SOLUTION_1_ANS);
    }

    println!("total size: {}", total_size);

    let MIN_UNUSED_SPACE: i32 = 30000000;
    let unused_space: i32 = 70000000 - total_size;
    let amt_to_free: i32 = MIN_UNUSED_SPACE - unused_space;
    println!(
        "unused_space: {} amt to free: {}",
        unused_space, amt_to_free
    );
    unsafe {
        use std::ops::Bound::*;
        let mut SOLUTION_2_ANS = SET.range((Included(amt_to_free as u32), Unbounded) ).next().unwrap();
        println!("solution 2: {}", SOLUTION_2_ANS);
    }

}

fn get_all_directory_size(node: NodeIndex, graph: &Graph<Node, ()>) -> u32 {
    let current_node = &graph[node];
    let mut total_weight = graph[node].size;
    for edge in graph.edges(node) {
        total_weight += get_all_directory_size(edge.target(), graph);
    }

    println!("{} {}", current_node.name, total_weight);
    if current_node.is_directory && total_weight <= SOLUTION_1_MAX_SIZE {
        // println!("directory: {}", current_node.name);

        unsafe {
            SOLUTION_1_ANS += total_weight;
        }
    }

    // solution 2
    if current_node.is_directory {
        unsafe {
            SET.insert(total_weight);
        }
    }

    total_weight
}

// do it without static
fn solution_2() {}
