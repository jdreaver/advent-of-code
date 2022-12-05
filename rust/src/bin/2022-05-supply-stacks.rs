use regex::Regex;

fn main() {
    let stacks = parse_stacks(INPUT_STACKS);
    let moves = parse_moves(INPUT_MOVES);
    println!("part 1: {}", part1_message(&stacks, &moves));
}

fn part1_message(stacks: &[Vec<char>], moves: &[Move]) -> String {
    part1_simulate_moves(stacks, moves)
        .iter()
        .map(|stack| {
            let stack_len = stack.len();
            stack[stack_len - 1]
        })
        .collect()
}

fn part1_simulate_moves(stacks: &[Vec<char>], moves: &[Move]) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = stacks.to_vec();
    for mv in moves {
        for _i in 0..mv.count {
            let elem = stacks[mv.start - 1].pop().unwrap();
            stacks[mv.end - 1].push(elem);
        }
    }
    stacks.to_vec()
}

#[derive(Debug)]
struct Move {
    count: u32,
    start: usize,
    end: usize,
}

fn parse_stacks(stacks: &str) -> Vec<Vec<char>> {
    stacks
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn parse_moves(input: &str) -> Vec<Move> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            Move {
                count: cap[1].parse().unwrap(),
                start: cap[2].parse().unwrap(),
                end: cap[3].parse().unwrap(),
            }
        })
        .collect()
}

// N.B. Manually parsed
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3
const _EXAMPLE_STACKS: &str = "ZN
MCD
P";

const _EXAMPLE_MOVES: &str = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

// N.B. Manually parsed
// [V]         [T]         [J]
// [Q]         [M] [P]     [Q]     [J]
// [W] [B]     [N] [Q]     [C]     [T]
// [M] [C]     [F] [N]     [G] [W] [G]
// [B] [W] [J] [H] [L]     [R] [B] [C]
// [N] [R] [R] [W] [W] [W] [D] [N] [F]
// [Z] [Z] [Q] [S] [F] [P] [B] [Q] [L]
// [C] [H] [F] [Z] [G] [L] [V] [Z] [H]
//  1   2   3   4   5   6   7   8   9

const INPUT_STACKS: &str = "CZNBMWQV
HZRWCB
FQRJ
ZSWHFNMT
GFWLNQP
LPW
VBDRGCQJ
ZQNBW
HLFCGTJ";

const INPUT_MOVES: &str = "move 2 from 1 to 7
move 6 from 2 to 6
move 10 from 7 to 6
move 4 from 3 to 1
move 5 from 6 to 4
move 1 from 1 to 9
move 4 from 6 to 9
move 12 from 4 to 1
move 5 from 1 to 4
move 7 from 9 to 8
move 11 from 8 to 1
move 6 from 6 to 2
move 2 from 5 to 2
move 3 from 6 to 3
move 4 from 9 to 4
move 2 from 2 to 5
move 1 from 6 to 4
move 3 from 3 to 6
move 1 from 8 to 4
move 1 from 6 to 1
move 28 from 1 to 4
move 28 from 4 to 5
move 1 from 9 to 1
move 4 from 4 to 1
move 2 from 6 to 2
move 2 from 1 to 6
move 7 from 4 to 2
move 14 from 2 to 9
move 1 from 4 to 1
move 1 from 1 to 2
move 18 from 5 to 6
move 2 from 2 to 6
move 1 from 9 to 7
move 8 from 9 to 2
move 15 from 6 to 5
move 1 from 6 to 3
move 3 from 2 to 5
move 1 from 7 to 5
move 2 from 1 to 3
move 3 from 2 to 1
move 1 from 6 to 4
move 5 from 6 to 5
move 2 from 2 to 9
move 35 from 5 to 7
move 4 from 9 to 3
move 1 from 4 to 1
move 5 from 1 to 7
move 6 from 5 to 3
move 1 from 9 to 4
move 11 from 7 to 6
move 2 from 9 to 2
move 1 from 4 to 7
move 14 from 7 to 4
move 5 from 6 to 9
move 2 from 2 to 4
move 6 from 7 to 9
move 2 from 9 to 5
move 6 from 9 to 5
move 8 from 4 to 9
move 5 from 4 to 3
move 3 from 5 to 7
move 1 from 3 to 9
move 5 from 3 to 4
move 7 from 9 to 8
move 2 from 7 to 4
move 4 from 5 to 7
move 1 from 5 to 3
move 5 from 6 to 4
move 8 from 4 to 8
move 5 from 7 to 6
move 1 from 4 to 7
move 3 from 6 to 9
move 2 from 6 to 5
move 7 from 8 to 3
move 2 from 5 to 9
move 17 from 3 to 1
move 3 from 1 to 3
move 6 from 8 to 9
move 4 from 4 to 7
move 6 from 3 to 5
move 2 from 8 to 5
move 14 from 7 to 5
move 2 from 4 to 5
move 6 from 9 to 5
move 1 from 7 to 9
move 1 from 6 to 9
move 8 from 1 to 9
move 8 from 5 to 2
move 2 from 1 to 3
move 7 from 2 to 6
move 2 from 3 to 4
move 1 from 2 to 6
move 3 from 1 to 6
move 16 from 9 to 4
move 2 from 9 to 8
move 1 from 1 to 6
move 2 from 9 to 4
move 1 from 6 to 9
move 1 from 6 to 1
move 1 from 1 to 7
move 1 from 6 to 9
move 1 from 9 to 3
move 1 from 3 to 8
move 1 from 9 to 2
move 1 from 2 to 7
move 2 from 5 to 3
move 7 from 5 to 8
move 2 from 7 to 9
move 1 from 6 to 7
move 3 from 6 to 9
move 10 from 8 to 7
move 1 from 4 to 3
move 3 from 3 to 1
move 1 from 7 to 1
move 19 from 4 to 6
move 3 from 9 to 7
move 1 from 9 to 2
move 2 from 1 to 7
move 1 from 9 to 1
move 12 from 6 to 9
move 2 from 7 to 1
move 1 from 2 to 4
move 11 from 6 to 3
move 1 from 4 to 8
move 1 from 6 to 8
move 11 from 7 to 9
move 2 from 8 to 9
move 18 from 9 to 6
move 5 from 3 to 7
move 5 from 3 to 8
move 11 from 5 to 6
move 26 from 6 to 4
move 1 from 6 to 5
move 1 from 3 to 7
move 3 from 8 to 3
move 1 from 8 to 7
move 3 from 3 to 6
move 5 from 9 to 3
move 1 from 4 to 9
move 8 from 4 to 5
move 2 from 7 to 8
move 3 from 3 to 6
move 3 from 4 to 6
move 7 from 7 to 4
move 1 from 9 to 1
move 5 from 5 to 3
move 2 from 9 to 7
move 3 from 8 to 2
move 7 from 3 to 7
move 1 from 7 to 6
move 3 from 5 to 6
move 7 from 4 to 8
move 10 from 4 to 5
move 2 from 4 to 2
move 3 from 7 to 5
move 2 from 4 to 1
move 6 from 8 to 5
move 5 from 1 to 4
move 5 from 4 to 2
move 5 from 7 to 8
move 10 from 2 to 8
move 3 from 8 to 3
move 2 from 5 to 3
move 13 from 6 to 1
move 19 from 5 to 3
move 12 from 3 to 9
move 4 from 8 to 2
move 2 from 6 to 7
move 5 from 8 to 7
move 9 from 3 to 9
move 1 from 5 to 9
move 2 from 7 to 6
move 3 from 2 to 3
move 15 from 9 to 3
move 13 from 3 to 5
move 1 from 6 to 2
move 5 from 5 to 8
move 1 from 2 to 5
move 1 from 7 to 6
move 6 from 9 to 6
move 6 from 6 to 8
move 4 from 7 to 1
move 2 from 3 to 6
move 11 from 1 to 9
move 1 from 2 to 3
move 4 from 5 to 6
move 1 from 1 to 6
move 10 from 9 to 2
move 8 from 2 to 3
move 3 from 1 to 2
move 8 from 3 to 1
move 5 from 5 to 4
move 1 from 9 to 8
move 2 from 3 to 7
move 2 from 4 to 5
move 6 from 1 to 6
move 9 from 8 to 1
move 16 from 1 to 9
move 2 from 7 to 3
move 3 from 3 to 8
move 6 from 9 to 6
move 1 from 5 to 4
move 1 from 3 to 8
move 5 from 2 to 1
move 5 from 1 to 9
move 2 from 4 to 9
move 4 from 8 to 6
move 1 from 8 to 7
move 4 from 8 to 5
move 2 from 8 to 2
move 17 from 9 to 5
move 11 from 5 to 7
move 1 from 2 to 5
move 1 from 2 to 5
move 1 from 9 to 1
move 1 from 1 to 6
move 5 from 7 to 6
move 20 from 6 to 7
move 4 from 6 to 4
move 15 from 7 to 8
move 2 from 3 to 7
move 1 from 6 to 5
move 10 from 8 to 4
move 1 from 3 to 6
move 4 from 6 to 4
move 13 from 7 to 8
move 1 from 7 to 5
move 1 from 6 to 3
move 1 from 6 to 3
move 1 from 6 to 9
move 9 from 4 to 1
move 3 from 8 to 2
move 14 from 5 to 6
move 2 from 2 to 8
move 1 from 3 to 9
move 14 from 6 to 2
move 1 from 3 to 9
move 1 from 9 to 3
move 15 from 2 to 1
move 1 from 3 to 9
move 4 from 4 to 9
move 10 from 8 to 5
move 1 from 9 to 5
move 1 from 1 to 5
move 4 from 8 to 7
move 3 from 9 to 3
move 1 from 8 to 5
move 1 from 4 to 7
move 2 from 8 to 7
move 6 from 5 to 6
move 4 from 1 to 2
move 1 from 2 to 5
move 2 from 2 to 8
move 2 from 8 to 1
move 3 from 7 to 2
move 3 from 4 to 9
move 18 from 1 to 8
move 1 from 7 to 3
move 3 from 9 to 6
move 1 from 1 to 5
move 5 from 6 to 4
move 2 from 1 to 9
move 8 from 4 to 5
move 4 from 3 to 2
move 16 from 5 to 4
move 8 from 8 to 6
move 2 from 2 to 6
move 1 from 7 to 6
move 7 from 8 to 1
move 1 from 2 to 3
move 2 from 8 to 3
move 4 from 4 to 9
move 4 from 1 to 2
move 1 from 7 to 2
move 1 from 5 to 4
move 1 from 3 to 7
move 3 from 4 to 5
move 1 from 9 to 6
move 9 from 2 to 5
move 2 from 3 to 6
move 3 from 5 to 8
move 3 from 1 to 7
move 4 from 5 to 8
move 1 from 4 to 3
move 5 from 9 to 5
move 5 from 5 to 8
move 1 from 3 to 4
move 4 from 5 to 1
move 2 from 5 to 4
move 13 from 6 to 2
move 12 from 2 to 9
move 3 from 9 to 2
move 4 from 1 to 6
move 8 from 6 to 2
move 1 from 4 to 9
move 3 from 7 to 9
move 2 from 9 to 8
move 1 from 7 to 2
move 9 from 9 to 5
move 2 from 8 to 6
move 4 from 2 to 3
move 1 from 7 to 2
move 1 from 6 to 4
move 4 from 3 to 9
move 9 from 5 to 8
move 10 from 4 to 2
move 1 from 4 to 7
move 1 from 6 to 2
move 1 from 6 to 7
move 13 from 2 to 6
move 1 from 2 to 5
move 6 from 6 to 5
move 7 from 5 to 8
move 1 from 4 to 5
move 27 from 8 to 5
move 3 from 6 to 3
move 2 from 8 to 6
move 8 from 9 to 5
move 1 from 7 to 9
move 1 from 6 to 2
move 4 from 5 to 9
move 2 from 3 to 4
move 9 from 2 to 5
move 1 from 4 to 1
move 1 from 4 to 2
move 1 from 2 to 4
move 1 from 3 to 7
move 1 from 1 to 3
move 1 from 3 to 9
move 6 from 9 to 4
move 1 from 7 to 5
move 13 from 5 to 2
move 1 from 9 to 5
move 1 from 7 to 2
move 5 from 2 to 7
move 8 from 5 to 7
move 6 from 4 to 2
move 1 from 4 to 5
move 3 from 2 to 4
move 4 from 2 to 7
move 2 from 4 to 3
move 13 from 7 to 3
move 5 from 2 to 3
move 4 from 7 to 8
move 11 from 3 to 8
move 11 from 5 to 9
move 4 from 6 to 9
move 1 from 6 to 5
move 1 from 4 to 2
move 1 from 3 to 6
move 3 from 2 to 6
move 3 from 6 to 2
move 1 from 6 to 1
move 1 from 3 to 8
move 3 from 3 to 6
move 2 from 2 to 7
move 4 from 3 to 9
move 16 from 9 to 2
move 1 from 7 to 8
move 2 from 2 to 8
move 9 from 2 to 3
move 6 from 2 to 7
move 1 from 6 to 3
move 2 from 9 to 2
move 1 from 9 to 7
move 2 from 6 to 3
move 4 from 3 to 9
move 2 from 2 to 7
move 1 from 2 to 5
move 14 from 5 to 6
move 14 from 6 to 3
move 4 from 9 to 8
move 5 from 8 to 4
move 1 from 1 to 5
move 4 from 8 to 1
move 1 from 5 to 9
move 8 from 7 to 2
move 18 from 3 to 7
move 1 from 1 to 5
move 1 from 1 to 9
move 1 from 4 to 5
move 1 from 8 to 5
move 8 from 2 to 9
move 3 from 5 to 8
move 7 from 7 to 1
move 3 from 4 to 7
move 1 from 3 to 6
move 7 from 8 to 3
move 2 from 9 to 3
move 3 from 8 to 9
move 9 from 1 to 7
move 9 from 3 to 4
move 2 from 3 to 4
move 12 from 7 to 4
move 1 from 3 to 8
move 1 from 8 to 7
move 8 from 4 to 7
move 11 from 4 to 9
move 5 from 4 to 8
move 19 from 7 to 9
move 1 from 6 to 2
move 2 from 7 to 4
move 2 from 8 to 3
move 1 from 7 to 8
move 1 from 3 to 2
move 3 from 8 to 4
move 1 from 8 to 9
move 1 from 3 to 2
move 36 from 9 to 1
move 5 from 9 to 6
move 5 from 4 to 2
move 24 from 1 to 3
move 5 from 6 to 7
move 1 from 1 to 4
move 14 from 3 to 4
move 4 from 7 to 3
move 1 from 8 to 5
move 5 from 2 to 9
move 1 from 1 to 6
move 5 from 9 to 1
move 3 from 2 to 3
move 1 from 5 to 3
move 11 from 4 to 2
move 1 from 7 to 1
move 6 from 1 to 9
move 3 from 4 to 2
move 1 from 6 to 7
move 10 from 1 to 7
move 3 from 2 to 1
move 3 from 3 to 2
move 2 from 1 to 7
move 1 from 4 to 8
move 13 from 3 to 2
move 1 from 8 to 3
move 2 from 7 to 5
move 2 from 3 to 7
move 2 from 5 to 2
move 1 from 1 to 7
move 28 from 2 to 6
move 1 from 2 to 3
move 2 from 8 to 2
move 6 from 9 to 7
move 1 from 3 to 8
move 1 from 9 to 8
move 3 from 6 to 2
move 14 from 7 to 9
move 3 from 2 to 1
move 2 from 2 to 9
move 2 from 1 to 9
move 1 from 9 to 1
move 7 from 6 to 9
move 2 from 1 to 4
move 2 from 4 to 6
move 4 from 8 to 7
move 1 from 7 to 6
move 1 from 8 to 1
move 1 from 3 to 6
move 1 from 1 to 5
move 14 from 9 to 8
move 1 from 5 to 9
move 5 from 7 to 3
move 16 from 6 to 3
move 2 from 7 to 4
move 8 from 9 to 5
move 6 from 6 to 1
move 8 from 5 to 9
move 2 from 7 to 4
move 11 from 9 to 1
move 4 from 4 to 1
move 14 from 8 to 3
move 2 from 1 to 7
move 20 from 3 to 6
move 5 from 3 to 1
move 1 from 3 to 5
move 2 from 7 to 4
move 20 from 6 to 7
move 18 from 7 to 6
move 17 from 6 to 9
move 1 from 5 to 3
move 6 from 3 to 2
move 3 from 3 to 1
move 1 from 6 to 2
move 2 from 7 to 8
move 4 from 1 to 5
move 2 from 4 to 9
move 1 from 3 to 2
move 1 from 8 to 6
move 18 from 1 to 4
move 1 from 2 to 7
move 1 from 6 to 2
move 3 from 4 to 3
move 1 from 8 to 1
move 4 from 1 to 6
move 7 from 2 to 1
move 1 from 5 to 7
move 1 from 4 to 1
move 2 from 6 to 3
move 3 from 5 to 9
move 9 from 9 to 8
move 10 from 9 to 3
move 9 from 3 to 5";
