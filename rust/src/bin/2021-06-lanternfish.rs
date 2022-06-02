use std::collections::HashMap;

fn main() {
    let input = parse_input(INPUT);
    println!("part1: {}", count_fish_after_days(&input, 80));
    println!("part1: {}", count_fish_after_days(&input, 256));
}

fn count_fish_after_days(input: &[usize], num_days: usize) -> u64 {
    // Populate counts
    let mut days_left_counts = vec![0; 9];
    for days_left in input {
        days_left_counts[*days_left] += 1;
    }

    // Simulate days
    for _ in 0..num_days {
        // Store new and reset fish in temp values
        let zero_fish = days_left_counts[0];
        for days_left in 1..=8 {
            days_left_counts[days_left - 1] = days_left_counts[days_left];
        }

        // Put new and reset fish back in
        days_left_counts[6] += zero_fish;
        days_left_counts[8] = zero_fish;
    }

    days_left_counts.iter().sum()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|c| c.parse::<usize>().expect("parse input usize"))
        .collect()
}

const _EXAMPLE: &str = "3,4,3,1,2";

const INPUT: &str = "3,5,2,5,4,3,2,2,3,5,2,3,2,2,2,2,3,5,3,5,5,2,2,3,4,2,3,5,5,3,3,5,2,4,5,4,3,5,3,2,5,4,1,1,1,5,1,4,1,4,3,5,2,3,2,2,2,5,2,1,2,2,2,2,3,4,5,2,5,4,1,3,1,5,5,5,3,5,3,1,5,4,2,5,3,3,5,5,5,3,2,2,1,1,3,2,1,2,2,4,3,4,1,3,4,1,2,2,4,1,3,1,4,3,3,1,2,3,1,3,4,1,1,2,5,1,2,1,2,4,1,3,2,1,1,2,4,3,5,1,3,2,1,3,2,3,4,5,5,4,1,3,4,1,2,3,5,2,3,5,2,1,1,5,5,4,4,4,5,3,3,2,5,4,4,1,5,1,5,5,5,2,2,1,2,4,5,1,2,1,4,5,4,2,4,3,2,5,2,2,1,4,3,5,4,2,1,1,5,1,4,5,1,2,5,5,1,4,1,1,4,5,2,5,3,1,4,5,2,1,3,1,3,3,5,5,1,4,1,3,2,2,3,5,4,3,2,5,1,1,1,2,2,5,3,4,2,1,3,2,5,3,2,2,3,5,2,1,4,5,4,4,5,5,3,3,5,4,5,5,4,3,5,3,5,3,1,3,2,2,1,4,4,5,2,2,4,2,1,4";
