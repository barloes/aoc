use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    hash::Hash,
    hash::Hasher,
    rc::Rc,
};

#[derive(Debug, Clone)]
struct Element {
    index: usize,
    value: i64,
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.value == other.value
    }
}

impl Eq for Element {}

impl Element {
    fn new(index: usize, value: i64) -> Element {
        Element { index, value }
    }
}

impl Hash for Element {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.value.hash(state);
    }
}

#[derive(Debug, Clone)]
struct Node {
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
    cur: Element,
}

impl Node {
    fn new(current: Element) -> Node {
        Node {
            prev: None,
            next: None,
            cur: current,
        }
    }

    fn get_next(&self) -> Rc<RefCell<Node>> {
        self.next.clone().expect("next is None")
    }

    fn get_prev(&self) -> Rc<RefCell<Node>> {
        self.prev.clone().expect("prev is None")
    }

    fn print(&self) {
        println!("index: {}, value:{}", self.cur.index, self.cur.value);
    }
}

#[derive(Debug)]
struct Game {
    original: Vec<Element>,
    // node: Rc<RefCell<Node>>,
    map: HashMap<Element, Rc<RefCell<Node>>>,
    len: i64,
}

impl Game {
    fn new(list: Vec<i64>) -> Game {
        let mut map = HashMap::new();

        let mut head = Rc::new(RefCell::new(Node::new(Element::new(0, list[0]))));
        map.insert(Element::new(0, list[0]), Rc::clone(&head));

        let mut prev = Rc::clone(&head);

        for i in 1..list.len() {
            let current_element = Element::new(i, list[i]);
            let node = Rc::new(RefCell::new(Node::new(current_element.clone())));

            node.borrow_mut().prev = Some(Rc::clone(&prev));
            prev.borrow_mut().next = Some(Rc::clone(&node));

            map.insert(current_element, Rc::clone(&node));
            prev = Rc::clone(&node);
        }

        head.borrow_mut().prev = Some(Rc::clone(&prev));
        prev.borrow_mut().next = Some(Rc::clone(&head));

        let element_list = list
            .iter()
            .enumerate()
            .map(|(i, v)| Element::new(i, *v))
            .collect::<Vec<Element>>();

        Game {
            original: element_list,
            map: map,
            len: list.len() as i64,
        }
    }
    // direction either 0 or 1
    fn cycle(&mut self, n: i64, mixing_round: i64) {
        let mut solution_1_ans: i64 = 0;
        for mix_round in 1..=mixing_round {
            for (element_index, element) in self.original.iter().enumerate() {
                if element.value == 0 {
                    continue;
                }
    
                let node_to_find = self.map.get(element).unwrap();
                let direction = if element.value > 0 { 1 } else { -1 };
                let mut visiting_node = match element.value > 0 {
                    true => node_to_find.borrow().get_prev(),
                    false => node_to_find.borrow().get_next(),
                };
                // remove node 
                {
                    let prev = node_to_find.borrow().get_prev();
                    let next = node_to_find.borrow().get_next();
                    prev.borrow_mut().next = Some(Rc::clone(&next));
                    next.borrow_mut().prev = Some(Rc::clone(&prev));
                    node_to_find.borrow_mut().prev = None;
                    node_to_find.borrow_mut().next = None;
                }
                let mut distance_to_travel = element.value.abs() % (self.len - 1);
                while distance_to_travel > 0 {
                    match direction {
                        1 => {
                            let next_node = visiting_node.borrow_mut().get_next();
                            visiting_node = next_node;
                        }
                        -1 => {
                            let prev_node = visiting_node.borrow_mut().get_prev();
                            visiting_node = prev_node;
                        }
                        _ => panic!("direction is not 1 or -1"),
                    }
                    distance_to_travel -= 1;
                }
                match direction {
                    1 => {
                        let to_update = visiting_node.borrow().get_next();
                        visiting_node.borrow_mut().next = Some(Rc::clone(&node_to_find));
                        to_update.borrow_mut().prev = Some(Rc::clone(&node_to_find));
                        node_to_find.borrow_mut().prev = Some(Rc::clone(&visiting_node));
                        node_to_find.borrow_mut().next = Some(Rc::clone(&to_update));
                    }
                    -1 => {
                        let to_update = visiting_node.borrow().get_prev();
                        visiting_node.borrow_mut().prev = Some(Rc::clone(&node_to_find));
                        to_update.borrow_mut().next = Some(Rc::clone(&node_to_find));
                        node_to_find.borrow_mut().next = Some(Rc::clone(&visiting_node));
                        node_to_find.borrow_mut().prev = Some(Rc::clone(&to_update));
                    }
                    _ => panic!("direction is not 1 or -1"),
                }
                self.map.insert(element.clone(), Rc::clone(&node_to_find));
            }
            self.print(1);
        }

        // self.node.borrow().print();
        let mut next_node = self
            .map
            .iter()
            .find(|(k, v)| k.value == 0)
            .unwrap()
            .1
            .clone();
        for i in 1..=n {
            let temp_node = next_node.borrow().get_next();
            next_node = Rc::clone(&temp_node);
            if i % 1000 == 0 {
                println!(
                    "cycle: {} element after zero: {}",
                    i,
                    next_node.borrow().clone().cur.value
                );
                solution_1_ans += next_node.borrow().clone().cur.value as i64;
            }
        }
        println!("solution 1: {}", solution_1_ans);
    }

    fn print(&self, multiplier: i64) {
        let starting_node = self.map.iter().find(|(k, v)| k.value == 0).unwrap().1;
        let starting_index = starting_node.borrow().cur.index;
        let mut count = 0;
        let mut node_to_visit = starting_node.clone().borrow().get_next();
        let mut vec_to_print = Vec::new();
        while count == 0 {
            vec_to_print.push(node_to_visit.borrow().cur.value as i64 * multiplier as i64);
            if node_to_visit.borrow().cur.index == starting_index {
                count += 1;
            }
            let next_node = node_to_visit.borrow().get_next();
            node_to_visit = next_node;
        }
        println!("{:?}", vec_to_print);
    }
}

fn main() {
    let mut game = read_input(1);
    game.cycle(3000, 1);
    
    let mut game = read_input(811589153);
    game.cycle(3000, 10);
}

fn read_input(multiplier: i64) -> Game {
    let mut list = Vec::new();
    let file = std::fs::read_to_string("input").unwrap();

    for line in file.lines() {
        list.push(line.parse::<i64>().unwrap() * multiplier);
    }


    Game::new(list)
}
