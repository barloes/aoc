use std::hash::{Hash, Hasher};
use std::{
    any::Any,
    cmp::max,
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fs,
    iter::Enumerate,
    num, usize,
};

use regex::Regex;

// static index for blueprint
const SLEEP: usize = 4;
const ORE_ROBOT: usize = 0;
const CLAY_ROBOT: usize = 1;
const OBSIDIAN_ROBOT: usize = 2;
const GEODE_ROBOT: usize = 3;

#[derive(Debug, Clone, Copy)]
struct BlueprintList {
    sleep_blueprint: SleepBlueprint,
    ore_robot_blueprint: OreRobotBlueprint,
    clay_robot_blueprint: ClayRobotBlueprint,
    obsidian_robot_blueprint: ObsidianRobotBlueprint,
    geode_robot_blueprint: GeodeRobotBlueprint,
}

impl BlueprintList {
    fn get_buildable(&self, state: &StateMachine) -> Vec<&dyn Blueprint> {
        let mut buildable: Vec<&dyn Blueprint> = Vec::new();

        if self.sleep_blueprint.is_buildable_from(*state) {
            buildable.push(&self.sleep_blueprint);
        }

        if self.ore_robot_blueprint.is_buildable_from(*state) {
            buildable.push(&self.ore_robot_blueprint);
        }

        if self.clay_robot_blueprint.is_buildable_from(*state) {
            buildable.push(&self.clay_robot_blueprint);
        }

        if self.obsidian_robot_blueprint.is_buildable_from(*state) {
            buildable.push(&self.obsidian_robot_blueprint);
        }

        buildable
    }
}

#[derive(Debug, Clone, Copy)]
struct SleepBlueprint {}

#[derive(Debug, Clone, Copy)]
struct OreRobotBlueprint {
    ore: u32,
}

#[derive(Debug, Clone, Copy)]
struct ClayRobotBlueprint {
    ore: u32,
}

#[derive(Debug, Clone, Copy)]
struct ObsidianRobotBlueprint {
    ore: u32,
    clay: u32,
}

#[derive(Debug, Clone, Copy)]
struct GeodeRobotBlueprint {
    ore: u32,
    obsidian: u32,
}

impl PartialEq for dyn Blueprint {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Eq for dyn Blueprint {}

impl Hash for dyn Blueprint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_id().hash(state);
    }
}

trait Blueprint: Any {
    fn build(&self, state: StateMachine) -> StateMachine;

    fn is_buildable_from(&self, state: StateMachine) -> bool;

    fn get_ore_requirement(&self) -> u32;

    fn get_clay_requirement(&self) -> u32;

    fn get_obsidian_requirement(&self) -> u32;

    fn get_geode_requirement(&self) -> u32;

    fn get_id(&self) -> usize;
}

impl Blueprint for SleepBlueprint {
    fn build(&self, state: StateMachine) -> StateMachine {
        state
    }

    fn is_buildable_from(&self, state: StateMachine) -> bool {
        true
    }

    fn get_ore_requirement(&self) -> u32 {
        0
    }

    fn get_clay_requirement(&self) -> u32 {
        0
    }

    fn get_obsidian_requirement(&self) -> u32 {
        0
    }

    fn get_geode_requirement(&self) -> u32 {
        0
    }

    fn get_id(&self) -> usize {
        SLEEP
    }
}

impl Blueprint for OreRobotBlueprint {
    fn build(&self, state: StateMachine) -> StateMachine {
        let mut new_state = state;

        new_state.ore_robot_number += 1;
        new_state.ore -= self.ore;
        new_state
    }

    fn is_buildable_from(&self, state: StateMachine) -> bool {
        state.ore >= self.ore
    }

    fn get_ore_requirement(&self) -> u32 {
        self.ore
    }

    fn get_clay_requirement(&self) -> u32 {
        0
    }

    fn get_obsidian_requirement(&self) -> u32 {
        0
    }

    fn get_geode_requirement(&self) -> u32 {
        0
    }

    fn get_id(&self) -> usize {
        ORE_ROBOT
    }
}

impl Blueprint for ClayRobotBlueprint {
    fn build(&self, state: StateMachine) -> StateMachine {
        let mut new_state = state;

        new_state.clay_robot_number += 1;
        new_state.ore -= self.ore;
        new_state
    }

    fn is_buildable_from(&self, state: StateMachine) -> bool {
        state.ore >= self.ore
    }

    fn get_ore_requirement(&self) -> u32 {
        self.ore
    }

    fn get_clay_requirement(&self) -> u32 {
        0
    }

    fn get_obsidian_requirement(&self) -> u32 {
        0
    }

    fn get_geode_requirement(&self) -> u32 {
        0
    }

    fn get_id(&self) -> usize {
        CLAY_ROBOT
    }
}

impl Blueprint for ObsidianRobotBlueprint {
    fn build(&self, state: StateMachine) -> StateMachine {
        let mut new_state = state;

        new_state.obsidian_robot_number += 1;
        new_state.clay -= self.clay;
        new_state.ore -= self.ore;
        new_state
    }

    fn is_buildable_from(&self, state: StateMachine) -> bool {
        state.clay >= self.clay && state.ore >= self.ore
    }

    fn get_ore_requirement(&self) -> u32 {
        self.ore
    }

    fn get_clay_requirement(&self) -> u32 {
        self.clay
    }

    fn get_obsidian_requirement(&self) -> u32 {
        0
    }

    fn get_geode_requirement(&self) -> u32 {
        0
    }

    fn get_id(&self) -> usize {
        OBSIDIAN_ROBOT
    }
}

impl Blueprint for GeodeRobotBlueprint {
    fn build(&self, state: StateMachine) -> StateMachine {
        let mut new_state = state;

        new_state.geode_robot_number += 1;
        new_state.obsidian -= self.obsidian;
        new_state.ore -= self.ore;
        new_state
    }

    fn is_buildable_from(&self, state: StateMachine) -> bool {
        state.obsidian >= self.obsidian && state.ore >= self.ore
    }

    fn get_ore_requirement(&self) -> u32 {
        self.ore
    }

    fn get_clay_requirement(&self) -> u32 {
        0
    }

    fn get_obsidian_requirement(&self) -> u32 {
        self.obsidian
    }

    fn get_geode_requirement(&self) -> u32 {
        0
    }

    fn get_id(&self) -> usize {
        GEODE_ROBOT
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct StateMachine {
    id: u32,
    ore_robot_number: u32,
    clay_robot_number: u32,
    obsidian_robot_number: u32,
    geode_robot_number: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    prev: Option<usize>, // index of blueprint 0: ore, 1: clay, 2: obsidian, 3: geode, 4: sleep
    time_left: u32,
}

impl StateMachine {
    fn new(id: u32, time_left: u32) -> StateMachine {
        StateMachine {
            id,
            ore_robot_number: 1,
            clay_robot_number: 0,
            obsidian_robot_number: 0,
            geode_robot_number: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            prev: None,
            time_left,
        }
    }

    fn generate_ore(&self) -> (u32, u32, u32, u32) {
        (
            self.ore_robot_number,
            self.clay_robot_number,
            self.obsidian_robot_number,
            self.geode_robot_number,
        )
    }

    //
    fn get_transitional_states(&self, blueprints: &BlueprintList) -> Vec<StateMachine> {
        let mut transitional_state = Vec::new();

        // only always build geode
        let recommendations = get_recommendations(self, blueprints);
        for blueprint in recommendations {
            let mut buildable_state = self.clone();
            // doing the lazy way of getting the transitional state
            while !blueprint.is_buildable_from(buildable_state) && buildable_state.time_left > 1 {
                buildable_state = buildable_state.rest();
            }

            if blueprint.is_buildable_from(buildable_state) && self.time_left > 0 {
                transitional_state.push(buildable_state.get_transitional_state(blueprint));
            }
        }

        transitional_state
    }

    fn rest(&self) -> StateMachine {
        let mut new_state = self.clone();
        new_state.time_left -= 1;
        let ore_gains = self.generate_ore();
        new_state.ore += ore_gains.0;
        new_state.clay += ore_gains.1;
        new_state.obsidian += ore_gains.2;
        new_state.geode += ore_gains.3;

        new_state
    }

    fn get_transitional_state(&self, blueprint: &dyn Blueprint) -> StateMachine {
        let ore_gains = self.generate_ore();
        let mut new_state = blueprint.build(*self);
        new_state.time_left -= 1;
        new_state.ore += ore_gains.0;
        new_state.clay += ore_gains.1;
        new_state.obsidian += ore_gains.2;
        new_state.geode += ore_gains.3;
        new_state
    }
}

fn main() {
    let blueprint_list = read_input();
    // get index along with the blueprint
    let mut solution_1_ans = 0;
    for (index, blueprints) in blueprint_list.iter().enumerate() {
        let state = StateMachine::new((index + 1) as u32, 24);
        let state_mx_geode = game_1(state, blueprints);

        solution_1_ans += state_mx_geode * (index + 1) as u32;
        println!("ans: {} quality: {}", solution_1_ans, state_mx_geode);
    }
    println!("Solution 1: {}", solution_1_ans);

    let mut solution_2_ans = 1;
    let first_three_blueprint = blueprint_list[0..3].to_vec();

    for (index, blueprints) in first_three_blueprint.iter().enumerate() {
        let state = StateMachine::new((index + 1) as u32, 32);
        let state_mx_geode = game_1(state, blueprints);

        solution_2_ans *= state_mx_geode;
        println!("ans: {} max_geode: {}", solution_2_ans, state_mx_geode);
    }
    println!("Solution 2: {}", solution_2_ans);
}

fn game_1(state: StateMachine, blueprints: &BlueprintList) -> u32 {
    let mut queue: Vec<_> = Vec::new();
    let mut visited = HashSet::new();
    queue.push(state);

    let mut mx_geode = 0;
    let mut space_taken = 0;
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        space_taken += 1;

        mx_geode = max(mx_geode, current.geode);
        for possible_state in current.get_transitional_states(blueprints) {
            if visited.contains(&possible_state) || !can_beat_geode_record(possible_state, mx_geode)
            {
                continue;
            }
            queue.push(possible_state);
            visited.insert(current);
        }
    }

    println!("space taken: {}", space_taken);
    mx_geode
}

fn read_input() -> Vec<BlueprintList> {
    // read from file delimeted by newline
    let input = fs::read_to_string("input").expect("Something went wrong reading the file");
    let input_list = input.split("\n");

    let mut blueprint_list = Vec::new();
    for instruction in input_list {
        let re = Regex::new(r"\d+").unwrap();
        let numbers: Vec<u32> = re
            .find_iter(instruction)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let ore_robot = OreRobotBlueprint { ore: numbers[1] };
        let clay_robot = ClayRobotBlueprint { ore: numbers[2] };
        let obsidian_robot = ObsidianRobotBlueprint {
            ore: numbers[3],
            clay: numbers[4],
        };
        let geode_robot = GeodeRobotBlueprint {
            ore: numbers[5],
            obsidian: numbers[6],
        };
        let do_nothing = SleepBlueprint {};
        let blueprint = BlueprintList {
            ore_robot_blueprint: ore_robot,
            clay_robot_blueprint: clay_robot,
            obsidian_robot_blueprint: obsidian_robot,
            geode_robot_blueprint: geode_robot,
            sleep_blueprint: do_nothing,
        };
        blueprint_list.push(blueprint);
        println!("{:?}", numbers);
    }
    blueprint_list
}

fn can_beat_geode_record(state: StateMachine, mx_geode: u32) -> bool {
    let time_left = state.time_left;
    let current_geode = state.geode;
    let optimum_geode =
        current_geode + summation(current_geode + time_left) - summation(current_geode);

    optimum_geode > mx_geode
}

fn summation(n: u32) -> u32 {
    n * (n + 1) / 2
}

// ore -> geode
// clay -> obsidian -> geode

/*
basically, if robot have enough supplier robots. this means that other robots are no longer needed
*/
/*
sleep optimisation
- no need to sleep if you can build everything.
-

 */

fn get_recommendations<'a>(
    state: &'a StateMachine,
    blueprints: &'a BlueprintList,
) -> Vec<&'a dyn Blueprint> {
    let mut recommendations: HashSet<&dyn Blueprint> = HashSet::new();
    recommendations.insert(&blueprints.geode_robot_blueprint);
    // recommendations.insert(&blueprints.sleep_blueprint);

    if have_more_robot_then_required(state, &blueprints.geode_robot_blueprint) {
        return vec![&blueprints.geode_robot_blueprint];
    } else {
        if state.obsidian_robot_number < blueprints.geode_robot_blueprint.get_obsidian_requirement()
        {
            recommendations.insert(&blueprints.obsidian_robot_blueprint);
            if !have_more_robot_then_required(state, &blueprints.obsidian_robot_blueprint) {
                if state.clay_robot_number
                    < blueprints.obsidian_robot_blueprint.get_clay_requirement()
                {
                    recommendations.insert(&blueprints.clay_robot_blueprint);
                    if !have_more_robot_then_required(state, &blueprints.clay_robot_blueprint) {
                        if state.ore_robot_number
                            < blueprints.clay_robot_blueprint.get_ore_requirement()
                        {
                            recommendations.insert(&blueprints.ore_robot_blueprint);
                        }
                    }
                }
                if state.ore_robot_number
                    < blueprints.obsidian_robot_blueprint.get_ore_requirement()
                {
                    recommendations.insert(&blueprints.ore_robot_blueprint);
                }
            }
        }

        if state.ore_robot_number < blueprints.geode_robot_blueprint.get_ore_requirement() {
            recommendations.insert(&blueprints.ore_robot_blueprint);
        }
    }

    recommendations.iter().map(|x| *x).collect()
}

fn have_more_robot_then_required(state: &StateMachine, blueprint: &dyn Blueprint) -> bool {
    if state.ore_robot_number >= blueprint.get_ore_requirement()
        && state.obsidian_robot_number >= blueprint.get_obsidian_requirement()
        && state.clay_robot_number >= blueprint.get_clay_requirement()
        && state.geode_robot_number >= blueprint.get_geode_requirement()
    {
        return true;
    }

    false
}