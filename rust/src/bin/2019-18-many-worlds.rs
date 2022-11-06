use core::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    let puzzle = parse_input(_EXAMPLE2);
    println!("{:?}", key_distances(&puzzle, puzzle.start.clone(), HashSet::new()));
    println!("{:?}", key_distances(&puzzle, puzzle.start.clone(), vec!['a', 'b', 'c', 'd'].into_iter().collect()));
}

#[derive(Debug, PartialEq, Eq)]
struct PuzzleState {
    distance: usize,
    position: Position,
    keys: HashSet<char>,
}

impl PartialOrd for PuzzleState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PuzzleState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

fn min_puzzle_distance(puzzle: &Puzzle) -> u32 {
    let start = PuzzleState {
        distance: 0,
        position: puzzle.start.clone(),
        keys: HashSet::new(),
    };

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(start));

    while let Some(state) = to_visit.pop() {}

    0
}

#[derive(Debug, PartialEq, Eq)]
struct KeyDistanceState {
    position: Position,
    distance: usize,
    seen_keys: HashSet<char>,
}

/// key_distances computes the distance from the current `location` to all of
/// the keys in the puzzle. The set of `current_keys` is used to determine which
/// doors we are allowed to pass through.
fn key_distances(
    puzzle: &Puzzle,
    location: Position,
    current_keys: HashSet<char>,
) -> HashMap<char, usize> {
    let mut explored: HashSet<Position> = HashSet::new();
    let mut pending: Vec<KeyDistanceState> = Vec::new();
    pending.push(KeyDistanceState {
        position: location,
        distance: 0,
        seen_keys: current_keys,
    });
    let mut distances: HashMap<char, usize> = HashMap::new();

    while let Some(state) = pending.pop() {
        // Check if we've been here before
        if explored.contains(&state.position) {
            continue;
        }
        explored.insert(state.position.clone());

        // Check for next steps
        for position in key_distance_next_4(puzzle, &state) {
            let distance = state.distance + 1;

            match puzzle.tiles[position.y][position.x] {
                Tile::Wall => continue,
                Tile::Key(c) => {
                    if let Some(&d) = distances.get(&c) {
                        distances.insert(c, std::cmp::min(d, distance));
                    } else {
                        distances.insert(c, distance);
                    }

                    let mut new_seen_keys = state.seen_keys.clone();
                    new_seen_keys.insert(c);
                    pending.push(KeyDistanceState {
                        position,
                        distance,
                        seen_keys: new_seen_keys,
                    });
                }
                Tile::Door(c) => {
                    if state.seen_keys.contains(&c) {
                        pending.push(KeyDistanceState {
                            position,
                            distance,
                            // TODO: This clone sucks and is probably bad for performance
                            seen_keys: state.seen_keys.clone(),
                        });
                    }
                }
                Tile::Empty => pending.push(KeyDistanceState {
                    position,
                    distance,
                    // TODO: This clone sucks and is probably bad for performance
                    seen_keys: state.seen_keys.clone(),
                }),
            }
        }
    }

    distances
}

fn key_distance_next_4(puzzle: &Puzzle, state: &KeyDistanceState) -> Vec<Position> {
    let mut positions = Vec::new();
    if state.position.x > 0 {
        positions.push(Position {
            x: state.position.x - 1,
            y: state.position.y,
        });
    }
    if state.position.y > 0 {
        positions.push(Position {
            x: state.position.x,
            y: state.position.y - 1,
        });
    }

    if state.position.x < puzzle.width - 1 {
        positions.push(Position {
            x: state.position.x + 1,
            y: state.position.y,
        });
    }

    if state.position.y < puzzle.height - 1 {
        positions.push(Position {
            x: state.position.x,
            y: state.position.y + 1,
        });
    }

    positions
}

#[derive(Debug)]
struct Puzzle {
    height: usize,
    width: usize,
    start: Position,
    keys: HashSet<char>,
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Tile {
    Wall,
    Key(char),
    Door(char),
    Empty,
}

fn parse_input(input: &str) -> Puzzle {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut start: Option<Position> = None;
    let mut keys = HashSet::new();

    let tiles = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '@' => {
                        start = Some(Position { x, y });
                        Tile::Empty
                    }
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'a'..='z' => {
                        keys.insert(c);
                        Tile::Key(c)
                    }
                    'A'..='Z' => Tile::Door(c.to_ascii_lowercase()),
                    _ => panic!("unknown char in input {} at ({}, {})", c, x, y),
                })
                .collect()
        })
        .collect();

    Puzzle {
        height: lines.len(),
        width: lines[0].len(),
        start: start.expect("couldn't find start tile"),
        keys,
        tiles,
    }
}

const _EXAMPLE1: &str = "#########
#b.A.@.a#
#########";

const _EXAMPLE2: &str = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
