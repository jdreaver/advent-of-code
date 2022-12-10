use std::ops::{Add, Sub};

fn main() {
    let input = parse_input(_EXAMPLE);
    println!("{:#?}", input);
    println!("{:#?}", simulate_moves(&input));
}

fn simulate_moves(moves: &[Move]) -> Vec<Rope> {
    let start = Rope::new();
    let tail = moves
        .iter()
        .scan(Rope::new(), |rope, mv| {
            println!("simulating {:?} w/ {:?}", rope, mv);
            *rope = rope.simulate_move(mv);
            Some(*rope)
        });
    let mut ropes = vec![start];
    ropes.extend(tail);
    ropes
}

#[derive(Debug, Clone, Copy)]
struct Rope {
    head: Point,
    tail: Point,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: Point{ x: 0, y: 0 },
            tail: Point{ x: 0, y: 0 },
        }
    }

    fn tail_position(&self) -> TailPosition {
        let diff = self.tail - self.head;
        match (diff.x, diff.y) {
            (1, 0) => TailPosition::Above,
            (1, 1) => TailPosition::AboveRight,
            (0, 1) => TailPosition::Right,
            (-1, 1) => TailPosition::BelowRight,
            (-1, 0) => TailPosition::Below,
            (-1, -1) => TailPosition::BelowLeft,
            (0, -1) => TailPosition::Left,
            (1, -1) => TailPosition::AboveLeft,
            (0, 0) => TailPosition::Covered,
            pos => panic!("invalid relative tail position {:?}", pos),
        }
    }

    fn simulate_move(&self, mv: &Move) -> Rope {
        let mut rope: Rope = self.clone();
        (0..mv.amount).for_each(|_| {
            rope = rope.simulate_single_move(&mv.direction);
        });
        println!("simulate_move output: {:?}", rope);
        rope
    }

    fn simulate_single_move(&self, direction: &MoveDirection) -> Rope {
        println!("simulate_single_move, self: {:?}, dir: {:?}", self, direction);
        Rope {
            head: self.head + head_move(direction),
            tail: self.tail + tail_move(&self.tail_position(), direction),
        }
    }
}

#[derive(Debug)]
enum TailPosition {
    Above,
    AboveRight,
    Right,
    BelowRight,
    Below,
    BelowLeft,
    Left,
    AboveLeft,
    Covered,
}

fn head_move(direction: &MoveDirection) -> Point {
    let (x, y) = match direction {
        MoveDirection::Up => (1, 0),
        MoveDirection::Right => (0, 1),
        MoveDirection::Down => (-1, 0),
        MoveDirection::Left => (0, -1),
    };

    Point { x, y }
}

fn tail_move(tail_pos: &TailPosition, head_direction: &MoveDirection) -> Point {
    let (x, y) = match (tail_pos, head_direction) {
        (TailPosition::Above, MoveDirection::Down) => (0, -1),
        (TailPosition::Above, _) => (0, 0),

        (TailPosition::Right, MoveDirection::Left) => (-1, 0),
        (TailPosition::Right, _) => (0, 0),

        (TailPosition::Below, MoveDirection::Up) => (1, 0),
        (TailPosition::Below, _) => (0, 0),

        (TailPosition::Left, MoveDirection::Right) => (0, 1),
        (TailPosition::Left, _) => (0, 0),

        (TailPosition::AboveRight, MoveDirection::Down) => (-1, -1),
        (TailPosition::AboveRight, MoveDirection::Left) => (-1, -1),
        (TailPosition::AboveRight, _) => (0, 0),

        (TailPosition::BelowRight, MoveDirection::Up) => (1, -1),
        (TailPosition::BelowRight, MoveDirection::Left) => (1, -1),
        (TailPosition::BelowRight, _) => (0, 0),

        (TailPosition::AboveLeft, MoveDirection::Down) => (-1, 1),
        (TailPosition::AboveLeft, MoveDirection::Right) => (-1, 1),
        (TailPosition::AboveLeft, _) => (0, 0),

        (TailPosition::BelowLeft, MoveDirection::Up) => (1, 1),
        (TailPosition::BelowLeft, MoveDirection::Right) => (1, 1),
        (TailPosition::BelowLeft, _) => (0, 0),

        // Tail never moves if covered
        (TailPosition::Covered, _) => (0, 0),
    };

    Point { x, y }
}

#[derive(Debug)]
struct Move {
    direction: MoveDirection,
    amount: u32,
}

#[derive(Debug)]
enum MoveDirection {
    Up,
    Right,
    Down,
    Left,
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let (dir_str, amount_str) = line.split_once(' ').expect("no space in line");
            let direction = match dir_str {
                "U" => MoveDirection::Up,
                "R" => MoveDirection::Right,
                "D" => MoveDirection::Down,
                "L" => MoveDirection::Left,
                _ => panic!("unexpected move direction {}", dir_str),
            };
            let amount = amount_str.parse::<u32>().expect("no parse amount");
            Move { direction, amount }
        })
        .collect()
}

const _EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
