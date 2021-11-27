use std::collections::HashMap;

fn main() {
    let input = parse_input(INPUT);
    println!("part1: {}", simulate_game(2020, &input));
    println!("part2: {}", simulate_game(30000000, &input));
}

fn simulate_game(stop: usize, starting: &[usize]) -> usize {
    let mut turn_prev_spoken: HashMap<usize, usize> = HashMap::new();
    let mut turn_last_spoken: HashMap<usize, usize> = HashMap::new();
    let mut last_spoken = 0;
    for (i, start) in starting.iter().enumerate() {
        turn_last_spoken.insert(*start, i + 1);
        last_spoken = *start;
    }

    for turn in (starting.len() + 1)..=stop {
        let new_last_spoken = match (turn_prev_spoken.get(&last_spoken), turn_last_spoken.get(&last_spoken)) {
            (None, None) => {
                0
            }
            (None, Some(_)) => {
                0
            }
            (Some(_), None) => panic!("internal error, found previous but not last for {}", last_spoken),
            (Some(&prev_turn), Some(&last_turn)) => {
                last_turn - prev_turn
            }
        };
        if let Some(&last_turn) = turn_last_spoken.get(&new_last_spoken) {
            turn_prev_spoken.insert(new_last_spoken, last_turn);
        }
        turn_last_spoken.insert(new_last_spoken, turn);
        last_spoken = new_last_spoken;
    }
    last_spoken
}

#[test]
fn test_simulate_game() {
    assert_eq!(simulate_game(2020, &parse_input("1,3,2")), 1);
    assert_eq!(simulate_game(2020, &parse_input("2,1,3")), 10);
    assert_eq!(simulate_game(2020, &parse_input("1,2,3")), 27);
    assert_eq!(simulate_game(2020, &parse_input("3,1,2")), 1836);
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|c| c.parse::<usize>().expect("parse input number"))
        .collect()
}

const _EXAMPLE: &str = "0,3,6";

const INPUT: &str = "0,13,1,16,6,17";
