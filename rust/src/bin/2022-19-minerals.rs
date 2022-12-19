use std::collections::{HashSet, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    IResult,
};

fn main() {
    let input = parse_blueprints(_EXAMPLE);
    println!("{:?}", input);

    println!("{}", maximize_geodes(&input[0], 24));
}

type Minutes = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SimulationState {
    remaining_minutes: Minutes,
    ore: Ore,
    ore_robots: u32,
    clay: Clay,
    clay_robots: u32,
    obsidian: Obsidian,
    obsidian_robots: u32,
    geodes: Geodes,
    geode_robots: u32,
}

struct SimulationQueue {
    seen_states: HashSet<SimulationState>,
    queue: VecDeque<SimulationState>,
}

impl SimulationQueue {
    fn new() -> Self {
        Self {
            seen_states: HashSet::new(),
            queue: VecDeque::new(),
        }
    }

    fn insert_state(&mut self, state: SimulationState) {
        if !self.seen_states.contains(&state) {
            self.queue.push_back(state.clone());
            self.seen_states.insert(state);
        }
    }
}

fn maximize_geodes(blueprint: &Blueprint, minutes: u32) -> Geodes {
    let mut max_geodes = 0;

    let mut queue = SimulationQueue::new();

    let initial_state = SimulationState {
            remaining_minutes: minutes,
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geodes: 0,
            geode_robots: 0,
        };
    queue.insert_state(initial_state);

    while let Some(state) = queue.queue.pop_front() {
        if state.remaining_minutes == 0 {
            max_geodes = std::cmp::max(max_geodes, state.geodes);
        }

        let new_state = SimulationState {
            remaining_minutes: state.remaining_minutes - 1,
            ore: state.ore + state.ore_robots,
            ore_robots: state.ore_robots,
            clay: state.clay + state.clay_robots,
            clay_robots: state.clay_robots,
            obsidian: state.obsidian + state.obsidian_robots,
            obsidian_robots: state.obsidian_robots,
            geodes: state.geodes + state.geode_robots,
            geode_robots: state.geode_robots,
        };
        queue.insert_state(new_state.clone());

        if state.ore >= blueprint.ore_robot_cost_ore {
            let mut state = state.clone();
            state.ore -= blueprint.ore_robot_cost_ore;
            state.ore_robots += 1;
            queue.insert_state(state);
        }

        if state.ore >= blueprint.clay_robot_cost_ore {
            let mut state = state.clone();
            state.ore -= blueprint.clay_robot_cost_ore;
            state.clay_robots += 1;
            queue.insert_state(state);
        }

        if state.ore >= blueprint.obsidian_robot_cost_ore && state.clay >= blueprint.obsidian_robot_cost_clay {
            let mut state = state.clone();
            state.ore -= blueprint.obsidian_robot_cost_ore;
            state.clay -= blueprint.obsidian_robot_cost_clay;
            state.obsidian_robots += 1;
            queue.insert_state(state);
        }

        if state.ore >= blueprint.geode_robot_cost_ore && state.obsidian >= blueprint.geode_robot_cost_obsidian {
            let mut state = state.clone();
            state.ore -= blueprint.geode_robot_cost_ore;
            state.obsidian -= blueprint.geode_robot_cost_obsidian;
            state.geode_robots += 1;
            queue.insert_state(state);
        }
    }

    max_geodes
}

type Ore = u32;
type Clay = u32;
type Obsidian = u32;
type Geodes = u32;

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost_ore: Ore,
    clay_robot_cost_ore: Ore,
    obsidian_robot_cost_ore: Ore,
    obsidian_robot_cost_clay: Clay,
    geode_robot_cost_ore: Ore,
    geode_robot_cost_obsidian: Obsidian,
}

fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let (remaining, blueprint) = parse_blueprint(line).expect("no parse input line");
            assert_eq!(remaining, "", "extra input left in line: {}", line);
            blueprint
        })
        .collect()
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = tag("Blueprint ")(input)?;
    let (input, _) = parse_u32(input)?;
    let (input, _) = tag(": Each ore robot costs ")(input)?;
    let (input, ore_robot_cost_ore) = parse_u32(input)?;
    let (input, _) = tag(" ore. Each clay robot costs ")(input)?;
    let (input, clay_robot_cost_ore) = parse_u32(input)?;
    let (input, _) = tag(" ore. Each obsidian robot costs ")(input)?;
    let (input, obsidian_robot_cost_ore) = parse_u32(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, obsidian_robot_cost_clay) = parse_u32(input)?;
    let (input, _) = tag(" clay. Each geode robot costs ")(input)?;
    let (input, geode_robot_cost_ore) = parse_u32(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, geode_robot_cost_obsidian) = parse_u32(input)?;
    let (input, _) = tag(" obsidian.")(input)?;
    Ok((
        input,
        Blueprint {
            ore_robot_cost_ore,
            clay_robot_cost_ore,
            obsidian_robot_cost_ore,
            obsidian_robot_cost_clay,
            geode_robot_cost_ore,
            geode_robot_cost_obsidian,
        },
    ))
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    let (rest, number) = map_res(digit1, |s: &str| s.parse())(input)?;
    Ok((rest, number))
}


const _EXAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

const INPUT: &str = "Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 15 clay. Each geode robot costs 2 ore and 8 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 3 ore and 10 obsidian.
Blueprint 3: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 14 obsidian.
Blueprint 4: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 4 ore and 15 obsidian.
Blueprint 5: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 6: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 7: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 9 clay. Each geode robot costs 3 ore and 7 obsidian.
Blueprint 8: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 8 obsidian.
Blueprint 9: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 18 obsidian.
Blueprint 10: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 2 ore and 19 obsidian.
Blueprint 11: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 3 ore and 10 obsidian.
Blueprint 12: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 2 ore and 16 obsidian.
Blueprint 13: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 16 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 14: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 13 obsidian.
Blueprint 15: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 2 ore and 20 obsidian.
Blueprint 16: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 4 ore and 10 obsidian.
Blueprint 17: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 17 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 18: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 17 obsidian.
Blueprint 19: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 20: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 21: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 22: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 3 ore and 19 obsidian.
Blueprint 23: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 24: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 13 clay. Each geode robot costs 2 ore and 9 obsidian.
Blueprint 25: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 26: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 27: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 16 obsidian.
Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 8 obsidian.
Blueprint 29: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 4 ore and 19 obsidian.
Blueprint 30: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 10 clay. Each geode robot costs 2 ore and 7 obsidian.";
