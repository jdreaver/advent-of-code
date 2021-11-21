fn main() {
    // println!("{}", part1_solution(5764801, 17807724)); // Test solution
    println!("{}", part1_solution(335121, 363891));

    // TODO: Part 2
}

fn part1_solution(card_pubkey: u64, door_pubkey: u64) -> u64 {
    let card_loop_size = find_loop_size(7, card_pubkey);
    transform_subject(door_pubkey, card_loop_size)
}

fn transform_subject(subject_number: u64, loop_size: usize) -> u64 {
    let mut result = 1;
    for _ in 0..loop_size {
        result = transform_iteration(result, subject_number);
    }
    result
}

#[inline]
fn transform_iteration(value: u64, subject_number: u64) -> u64 {
    (value * subject_number).rem_euclid(20201227)
}

fn find_loop_size(subject_number: u64, public_key: u64) -> usize {
    let mut value = 1;
    let mut loop_size = 0;
    loop {
        loop_size += 1;
        value = transform_iteration(value, subject_number);
        if value == public_key {
            return loop_size;
        }
    }
}
