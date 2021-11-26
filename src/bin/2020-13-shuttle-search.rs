fn main() {
    let input = parse_input(_EXAMPLE);
    println!("part1: {}", first_bus(&input));
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
