use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn get_neighbours(&self) -> Vec<Cube> {
        let mut neighbours = Vec::new();

        for offset in [-1, 1].iter() {
            neighbours.push(Cube {
                x: self.x + offset,
                y: self.y,
                z: self.z,
            });
            neighbours.push(Cube {
                x: self.x,
                y: self.y + offset,
                z: self.z,
            });
            neighbours.push(Cube {
                x: self.x,
                y: self.y,
                z: self.z + offset,
            });
        }

        neighbours.iter().map(|cube| *cube).collect()
    }
}

struct Game {
    cube_set: HashSet<Cube>,
    visited: HashSet<Cube>,
}

impl Game {
    fn new() -> Game {
        let cube_set = read_input();

        Game {
            cube_set,
            visited: HashSet::new(),
        }
    }

    fn get_all_cubes(&self) -> Vec<Cube> {
        self.cube_set.iter().map(|cube| *cube).collect()
    }

    fn generate_surface_area(&mut self, cube: Cube, wanted_neighbours: &HashSet<Cube>) -> i32 {
        if self.visited.contains(&cube) {
            return 0;
        }
        self.visited.insert(cube);
        let good_neighbours = self.get_neighbours(&cube, true, wanted_neighbours);
        let mut total_surface_area = 6 - good_neighbours.len() as i32;
        for nxt_cube in good_neighbours {
            total_surface_area += self.generate_surface_area(nxt_cube, wanted_neighbours);
        }
        total_surface_area
    }

    fn get_neighbours(
        &self,
        cube: &Cube,
        is_inside: bool,
        wanted_neighbours: &HashSet<Cube>,
    ) -> Vec<Cube> {
        let mut neighbours = Vec::new();

        for offset in [-1, 1].iter() {
            neighbours.push(Cube {
                x: cube.x + offset,
                y: cube.y,
                z: cube.z,
            });
            neighbours.push(Cube {
                x: cube.x,
                y: cube.y + offset,
                z: cube.z,
            });
            neighbours.push(Cube {
                x: cube.x,
                y: cube.y,
                z: cube.z + offset,
            });
        }

        if is_inside {
            return neighbours
                .iter()
                .filter(|cube| wanted_neighbours.contains(cube))
                .map(|cube| *cube)
                .collect();
        } else {
            return neighbours
                .iter()
                .filter(|cube| !wanted_neighbours.contains(cube))
                .map(|cube| *cube)
                .collect();
        }
    }
}

struct Game2 {
    cube_set: HashSet<Cube>,
    min_x: i32,
    min_y: i32,
    min_z: i32,
    max_x: i32,
    max_y: i32,
    max_z: i32,
}

impl Game2 {
    fn new() -> Game2 {
        let cube_set = read_input();

        // get min in cube set
        let min_x = cube_set.iter().min_by_key(|cube| cube.x).unwrap().x - 1;
        let min_y = cube_set.iter().min_by_key(|cube| cube.y).unwrap().y - 1;
        let min_z = cube_set.iter().min_by_key(|cube| cube.z).unwrap().z - 1;

        let mx_x = cube_set.iter().max_by_key(|cube| cube.x).unwrap().x + 1;
        let mx_y = cube_set.iter().max_by_key(|cube| cube.y).unwrap().y + 1;
        let mx_z = cube_set.iter().max_by_key(|cube| cube.z).unwrap().z + 1;

        Game2 {
            cube_set,
            min_x: min_x,
            min_y: min_y,
            min_z: min_z,
            max_x: mx_x,
            max_y: mx_y,
            max_z: mx_z,
        }
    }

    fn traverse(&self) -> i32 {
        let good_neighbour_set = self.cube_set.iter().cloned().collect::<HashSet<Cube>>();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut count = 0;
        let starting = Cube {
            x: self.min_x,
            y: self.min_y,
            z: self.min_z,
        };
        queue.push_back(starting);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);
            let neighbours_to_visit = current.get_neighbours();

            count += neighbours_to_visit
                .iter()
                .filter(|cube| good_neighbour_set.contains(cube))
                .count() as i32;

            for neighbour in neighbours_to_visit.iter().filter(|cube| {
                !self.is_out_of_bound(
                    current.x, current.y, current.z
                ) && !good_neighbour_set.contains(cube)
            }) {
                queue.push_back(*neighbour);
            }
        }

        count
    }
    fn is_out_of_bound(&self, x: i32, y: i32, z: i32) -> bool {
        x < self.min_x
            || x > self.max_x
            || y < self.min_y
            || y > self.max_y
            || z < self.min_z
            || z > self.max_z
    }
}

fn main() {
    let mut game = Game::new();
    let cubes = game.get_all_cubes();
    let good_neighbour_set = cubes.iter().cloned().collect::<HashSet<Cube>>();
    let mut total_surface_area = 0;
    for cube in cubes.clone() {
        total_surface_area += game.generate_surface_area(cube, &good_neighbour_set);
    }
    println!("solution 1: Total surface area: {}", total_surface_area);

    let game2 = Game2::new();
    let solution_2 = game2.traverse();
    println!("solution 2: Total surface area: {}", solution_2);
}

fn read_input() -> HashSet<Cube> {
    let mut cube_set = HashSet::new();

    let file = File::open("input").expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // split by , convert to i32
        let line = line
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let cube = Cube {
            x: line[0],
            y: line[1],
            z: line[2],
        };
        cube_set.insert(cube);
    }

    cube_set
}
