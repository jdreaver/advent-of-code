fn main() {
    let input = parse_input(INPUT);
    println!("part1: {}", first_bus(&input));
    println!("part2: {}", first_consecutive_depart_timestamp(&input.bus_ids));
}

fn first_bus(input: &Input) -> u32 {
    let (min_wait, id) = input
        .bus_ids
        .iter()
        .filter_map(|id| *id)
        .map(|id| (id - (input.earliest_departure.rem_euclid(id)), id))
        .min()
        .expect("couldn't find first bus");
    min_wait * id
}

fn first_consecutive_depart_timestamp(bus_ids: &[Option<u32>]) -> u64 {
    let ids_offsets = bus_ids
        .iter()
        .enumerate()
        .filter_map(|(offset, mid)| mid.map(|id| (offset, id as u64)))
        .collect::<Vec<(usize, u64)>>();

    let mut step_size = ids_offsets[0].1;
    let mut t = 0;
    for (offset, id) in ids_offsets.iter().skip(1) {
        while (t + *offset as u64).rem_euclid(*id) != 0 {
            t += step_size;
        }
        step_size *= id;
    }
    t
}

#[derive(Debug)]
struct Input {
    earliest_departure: u32,
    bus_ids: Vec<Option<u32>>,
}

fn parse_input(input: &str) -> Input {
    let lines = input.lines().collect::<Vec<&str>>();
    let earliest_departure = lines[0].parse::<u32>().expect("parse u32");
    let bus_ids = lines[1]
        .split(",")
        .map(|c| match c {
            "x" => None,
            _ => Some(c.parse::<u32>().expect("bus id"))
        })
        .collect();

    Input { earliest_departure, bus_ids }
}

const _EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";

const INPUT: &str = "1006697
13,x,x,41,x,x,x,x,x,x,x,x,x,641,x,x,x,x,x,x,x,x,x,x,x,19,x,x,x,x,17,x,x,x,x,x,x,x,x,x,x,x,29,x,661,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23";
