use std::{collections::VecDeque, vec};

enum Effect {
    Noop,
    AddX(i32),
}

struct PixelGraph {
    x: i32,
    graph: Vec<char>,
}

impl PixelGraph {
    fn new() -> PixelGraph {
        PixelGraph {
            graph: vec![],
            x: 0,
        }
    }

    fn add_pixel(&mut self, sprite_position: (i32, i32)) {
        let (from, to) = sprite_position;

        if self.x < from || self.x > to {
            self.graph.push('.');
        } else {
            self.graph.push('#');
        }
        self.x += 1;
        if self.x == 40 {
            self.x = 0;
        }
    }

    fn print(&self) {
        for i in 0..self.graph.len() {
            print!("{} ", self.graph[i]);
            if (i + 1) % 40 == 0 {
                println!();
            }
        }
    }
}

struct Game {
    cycle: i32,
    signal_strength: i32,
    queue: VecDeque<Effect>,
}

impl Game {
    fn new() -> Game {
        Game {
            cycle: 1,
            signal_strength: 1,
            queue: VecDeque::new(),
        }
    }

    fn simulate_v2(&mut self) {
        let mut sprite_from = 0;
        let mut sprite_to = 2;
        let mut pixel_graph = PixelGraph::new();

        let mut index = 0;

        // readfile
        let file = std::fs::read_to_string("input").unwrap();
        for line in file.lines() {
            let (expected_signal_strength, expected_cycle) = self.parse_command(line);
            for _ in 0..expected_cycle {
                pixel_graph.add_pixel((sprite_from, sprite_to));
            }
            index = index + expected_cycle;

            sprite_from += expected_signal_strength;
            sprite_to += expected_signal_strength;
        }

        pixel_graph.print();
    }

    fn simulate(&mut self) {
        // 20, 60, 100, 140, 180, 220
        let mut ptr = 0;
        let interesting_cycle_list = [20, 60, 100, 140, 180, 220];
        let mut solution_1_ans = 0;
        let mut track_list: Vec<i32> = vec![];
        while self.queue.len() > 0 {
            let effect = self.queue.pop_front().unwrap();

            let (expected_signal_strength, expected_cycle);

            match effect {
                Effect::Noop => {
                    (expected_signal_strength, expected_cycle) = self.get_noop_expected()
                }
                Effect::AddX(value_to_add) => {
                    (expected_signal_strength, expected_cycle) =
                        self.get_addx_expected(value_to_add)
                }
            }

            // check that ptr not out of bound and
            if ptr < interesting_cycle_list.len() && interesting_cycle_list[ptr] < expected_cycle {
                solution_1_ans += self.signal_strength * interesting_cycle_list[ptr];
                ptr += 1;
                track_list.push(self.signal_strength);
            }

            self.cycle = expected_cycle;
            self.signal_strength = expected_signal_strength;
        }

        println!("Track list: {:?}", track_list);
        println!("Solution 1: {}", solution_1_ans);
    }

    // get expected signal strength and cycle
    fn parse_command(&mut self, command: &str) -> (i32, i32) {
        let mut split = command.split_whitespace();

        // get effect timeline
        // apply effect to game state

        match split.next().unwrap() {
            "noop" => {
                self.queue.push_back(Effect::Noop);
                return (0, 1);
            }
            "addx" => {
                let value_to_add = split.next().unwrap().parse::<i32>().unwrap();
                self.queue.push_back(Effect::AddX(value_to_add));
                return (value_to_add, 2);
            }
            _ => panic!("Unknown command"),
        }
    }

    // return expected signal strenght and cycle
    fn get_noop_expected(&mut self) -> (i32, i32) {
        (self.signal_strength, self.cycle + 1)
    }

    fn get_addx_expected(&mut self, value_to_add: i32) -> (i32, i32) {
        (value_to_add + self.signal_strength, self.cycle + 2)
    }
}

fn main() {
    // read file
    let file = std::fs::read_to_string("input").unwrap();
    let mut game = Game::new();

    for line in file.lines() {
        game.parse_command(line);
    }
    game.simulate();
    game.simulate_v2();
}
