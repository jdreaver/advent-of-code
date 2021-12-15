use itertools::Itertools;

fn main() {
    let input = parse_input(INPUT);

    let part1_next_values = simulate_turns(&input, 100);
    let mut clockwise_from_one: Vec<usize> = vec![part1_next_values[1]];
    for _ in 0..(input.len() - 2) {
        clockwise_from_one.push(part1_next_values[*clockwise_from_one.last().unwrap()]);
    }
    let part1: String = clockwise_from_one.iter().map(|x| x.to_string()).collect();
    println!("part1: {}", part1);

    let mut part2_input = input.clone();
    for i in (input.len() + 1)..=1000000 {
        part2_input.push(i as u32);
    }
    let part2_next_values = simulate_turns(&part2_input, 10000000);
    let after_1 = part2_next_values[1];
    let after_after_1 = part2_next_values[after_1];
    let part2 = after_1 * after_after_1;
    println!("part2: {}", part2);
}

fn simulate_turns(input: &[u32], turns: usize) -> Vec<usize> {
    // next_value[i] is the value that comes after i
    let mut next_value: Vec<usize> = vec![0; input.len() + 1];
    for i in 0..(input.len() - 1) {
        next_value[input[i] as usize] = input[i + 1] as usize;
    }
    next_value[input[input.len() - 1] as usize] = input[0] as usize; // Wrap around
    let mut current: usize = input[0] as usize;

    for _ in 0..turns {
        // Pick up the next 3 cups
        let mut picked_up = vec![next_value[current]];
        picked_up.push(next_value[picked_up[0]]);
        picked_up.push(next_value[picked_up[1]]);

        // Find the destination cup
        let mut dest = current - 1;
        while dest == 0 || picked_up.iter().any(|x| *x == dest) {
            if dest == 0 {
                dest = input.len();
            } else {
                dest -= 1;
            }
        }

        // Put cups back
        next_value[current] = next_value[picked_up[2]];
        next_value[picked_up[2]] = next_value[dest];
        next_value[dest] = picked_up[0];
        current = next_value[current];
    }

    next_value
}

// Old, slow solution
//
// fn part1_solution(input: &[u32]) -> String {
//     let cups = simulate_turns(input, 100);
//     let one_index = cups.iter().position(|c| *c == 1).unwrap();
//     let mut clockwise_from_one = Vec::new();
//     for i in 1..input.len() {
//         clockwise_from_one.push(cups[(i + one_index).rem_euclid(input.len())]);
//     }
//     clockwise_from_one
//         .iter()
//         .map(|x| x.to_string())
//         .collect()
// }

// fn simulate_turns(input: &[u32], turns: usize) -> Vec<u32> {
//     let mut cups = input.to_vec();
//     for _ in 0..turns {
//         cups = simulate_turn(&cups);
//     }
//     cups
// }

// fn simulate_turn(cups: &[u32]) -> Vec<u32> {
//     // Find the label of the destination cup: the cup with a label
//     // equal to the current cup's label minus one. If this would
//     // select one of the cups that was just picked up, the crab
//     // will keep subtracting one until it finds a cup that wasn't
//     // just picked up. If at any point in this process the value
//     // goes below the lowest value on any cup's label, it wraps
//     // around to the highest value on any cup's label instead.
//     let mut dest_label = cups[0] - 1;
//     loop {
//         if dest_label == 0 {
//             // Cups are numbered from 1 to N. Wrap around to N.
//             dest_label = cups.len() as u32;
//         }
//         if cups[4..].iter().any(|x| *x == dest_label) {
//             break;
//         }
//         dest_label -= 1;
//     }

//     // Keep the current cup at the front of the array by rotating
//     // all elements left once, and also insert all of the old cups
//     // into this new array: The crab places the cups it just
//     // picked up so that they are immediately clockwise of the
//     // destination cup. They keep the same order as when they were
//     // picked up.
//     let mut new_cups = Vec::new();
//     for i in 4..cups.len() {
//         new_cups.push(cups[i]);
//         if cups[i] == dest_label {
//             for i in 1..=3 {
//                 new_cups.push(cups[i]);
//             }
//         }
//     }
//     new_cups.push(cups[0]);
//     new_cups
// }

fn parse_input(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|c| c.to_digit(10).expect("parse input num"))
        .collect()
}

const _EXAMPLE: &str = "389125467";

const INPUT: &str = "253149867";
