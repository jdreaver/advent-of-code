fn main() {
    let input = parse_input(INPUT);

    let part1 = part1_solution(&input);
    println!("{}", part1);
}

fn part1_solution(input: &[u32]) -> String {
    let cups = simulate_turns(input, 100);
    let one_index = cups.iter().position(|c| *c == 1).unwrap();
    let mut clockwise_from_one = Vec::new();
    for i in 1..input.len() {
        clockwise_from_one.push(cups[(i + one_index).rem_euclid(input.len())]);
    }
    clockwise_from_one
        .iter()
        .map(|x| x.to_string())
        .collect()
}

fn simulate_turns(input: &[u32], turns: usize) -> Vec<u32> {
    let mut cups = input.to_vec();
    for _ in 0..turns {
        cups = simulate_turn(&cups);
    }
    cups
}

fn simulate_turn(cups: &[u32]) -> Vec<u32> {
    // Find the label of the destination cup: the cup with a label
    // equal to the current cup's label minus one. If this would
    // select one of the cups that was just picked up, the crab
    // will keep subtracting one until it finds a cup that wasn't
    // just picked up. If at any point in this process the value
    // goes below the lowest value on any cup's label, it wraps
    // around to the highest value on any cup's label instead.
    let mut dest_label = cups[0] - 1;
    loop {
        if dest_label == 0 {
            // Cups are numbered from 1 to N. Wrap around to N.
            dest_label = cups.len() as u32;
        }
        if cups[4..].iter().any(|x| *x == dest_label) {
            break;
        }
        dest_label -= 1;
    }

    // Keep the current cup at the front of the array by rotating
    // all elements left once, and also insert all of the old cups
    // into this new array: The crab places the cups it just
    // picked up so that they are immediately clockwise of the
    // destination cup. They keep the same order as when they were
    // picked up.
    let mut new_cups = Vec::new();
    for i in 4..cups.len() {
        new_cups.push(cups[i]);
        if cups[i] == dest_label {
            for i in 1..=3 {
                new_cups.push(cups[i]);
            }
        }
    }
    new_cups.push(cups[0]);
    new_cups
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|c| c.to_digit(10).expect("parse input num"))
        .collect()
}

const _EXAMPLE: &str = "389125467";

const INPUT: &str = "253149867";
