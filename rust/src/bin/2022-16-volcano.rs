use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map_res, opt},
    multi::separated_list1,
    IResult,
};

fn main() {
    let tunnels = parse_input(_EXAMPLE);
    println!("{:#?}", tunnels);
}

#[derive(Debug)]
struct Tunnel {
    name: String,
    flow_rate: u32,
    connections: Vec<String>,
}

fn parse_input(input: &str) -> Vec<Tunnel> {
    input
        .lines()
        .map(|line| {
            let (remaining, reading) = parse_input_line(line).expect("no parse input line");
            assert_eq!(remaining, "", "extra input left in line: {}", line);
            reading
        })
        .collect()
}

fn parse_input_line(input: &str) -> IResult<&str, Tunnel> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = parse_u32(input)?;
    let (input, _) = tag("; tunnel")(input)?;
    let (input, _) = opt(tag("s"))(input)?;
    let (input, _) = tag(" lead")(input)?;
    let (input, _) = opt(tag("s"))(input)?;
    let (input, _) = tag(" to valve")(input)?;
    let (input, _) = opt(tag("s"))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, connections) = separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Tunnel {
            name: name.to_string(),
            flow_rate,
            connections: connections.iter().map(|c| c.to_string()).collect(),
        },
    ))
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    let (rest, number) = map_res(digit1, |s: &str| s.parse())(input)?;
    Ok((rest, number))
}

const _EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
