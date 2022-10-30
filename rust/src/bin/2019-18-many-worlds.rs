use core::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

fn main() {
    println!("{:?}", parse_input(_example1));
}

#[derive(Debug, PartialEq, Eq)]
struct PuzzleState {
    distance: usize,
    position: (usize, usize),
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
        position: puzzle.start,
        keys: HashSet::new(),
    };

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(start));

    while let Some(state) = to_visit.pop() {

    }

    0
}

#[derive(Debug)]
struct Puzzle {
    height: usize,
    width: usize,
    start: (usize, usize),
    tiles: Vec<Vec<Tile>>,
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
    let mut start: Option<(usize, usize)> = None;

    let tiles = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '@' => {
                        start = Some((i, j));
                        Tile::Empty
                    }
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'a'..='z' => Tile::Key(c),
                    'A'..='Z' => Tile::Door(c),
                    _ => panic!("unknown char in input {} at ({}, {})", c, i, j),
                })
                .collect()
        })
        .collect();

    Puzzle {
        height: lines.len(),
        width: lines[0].len(),
        start: start.expect("couldn't find start tile"),
        tiles,
    }
}

const _example1: &str = "#########
#b.A.@.a#
#########";
