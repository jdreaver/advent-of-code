use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map_res, opt},
    multi::separated_list1,
    IResult,
};

fn main() {
    let tunnels = parse_input(_EXAMPLE);
    println!("part 1: {}", max_pressure_release(&tunnels));
}

type TotalPressure = u32;

fn max_pressure_release(tunnels: &[Tunnel], turns: u32) -> TotalPressure {
    let tunnels_by_name: HashMap<&String, &Tunnel> = tunnels
        .iter()
        .map(|tunnel| (&tunnel.name, tunnel))
        .collect();

    let distances = tunnel_distances(tunnels);

    let mut paths: HashMap<Path, TotalPressure> = HashMap::new();
    let mut queue: VecDeque<(Path, TotalPressure)> = VecDeque::new();
    queue.push_back((
        Path {
            tunnel: "AA".to_string(),
            opened_valves: BTreeSet::new(),
        },
        0,
    ));

    while let Some((path, total_pressure)) = queue.pop_front() {
        let tunnel = tunnels_by_name.get(&path.tunnel).expect("no tunnel found");
        // if let Some(&existing_pressure) = paths.get(&path) {
        //     if existing_pressure <= total_pressure {
        //         // We already have a better path here
        //         continue;
        //     }
        // }
    }

    0
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Path {
    tunnel: String,
    opened_valves: BTreeSet<String>,
}

type Steps = u32;

// Dijkstra to find distances between every pair of tunnels
fn tunnel_distances(tunnels: &[Tunnel]) -> HashMap<(String, String), Steps> {
    tunnels
        .iter()
        .flat_map(|source| {
            tunnels
                .iter()
                // We only care about the distance to tunnels with a positive
                // flow rate.
                .filter(move |&dest| dest.name != source.name && dest.flow_rate > 0)
                .map(move |dest| {
                    (
                        (source.name.clone(), dest.name.clone()),
                        tunnel_distance(tunnels, &source.name, &dest.name).expect("no path found!"),
                    )
                })
        })
        .collect()
}

fn tunnel_distance(tunnels: &[Tunnel], start: &String, target: &String) -> Option<Steps> {
    let tunnels_by_name: HashMap<&String, &Tunnel> = tunnels
        .iter()
        .map(|tunnel| (&tunnel.name, tunnel))
        .collect();

    let mut visited: HashMap<String, Steps> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(Steps, String)>> = BinaryHeap::new();
    queue.push(Reverse((0, start.clone())));

    while let Some(Reverse((steps, tunnel))) = queue.pop() {
        if let Some(&existing_steps) = visited.get(&tunnel) {
            if existing_steps <= steps {
                // We already have a better path to this node
                continue;
            }
        }
        visited.insert(tunnel.clone(), steps);

        let adjacent = &tunnels_by_name
            .get(&tunnel)
            .expect("tunnel not found")
            .connections;
        for next in adjacent.iter() {
            if next == target {
                return Some(steps + 1);
            }
            queue.push(Reverse((steps + 1, next.clone())));
        }
    }

    None
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

const INPUT: &str = "Valve TM has flow rate=3; tunnels lead to valves WB, PE, DX, TK, CH
Valve ST has flow rate=21; tunnels lead to valves NS, DE, UX, XU
Valve IX has flow rate=0; tunnels lead to valves DK, LR
Valve OG has flow rate=0; tunnels lead to valves MN, FK
Valve FR has flow rate=0; tunnels lead to valves JQ, GS
Valve HU has flow rate=0; tunnels lead to valves TJ, XX
Valve WC has flow rate=15; tunnel leads to valve TJ
Valve JT has flow rate=0; tunnels lead to valves OV, AA
Valve DW has flow rate=0; tunnels lead to valves FK, AA
Valve RG has flow rate=0; tunnels lead to valves PS, DK
Valve JQ has flow rate=14; tunnels lead to valves VM, FR
Valve XX has flow rate=5; tunnels lead to valves GP, MN, WB, LM, HU
Valve IN has flow rate=11; tunnels lead to valves OK, GS, DU
Valve LR has flow rate=7; tunnels lead to valves IX, NR, YY, HZ, PR
Valve TK has flow rate=0; tunnels lead to valves TM, OV
Valve VM has flow rate=0; tunnels lead to valves KQ, JQ
Valve IC has flow rate=0; tunnels lead to valves FK, DU
Valve CH has flow rate=0; tunnels lead to valves EZ, TM
Valve OV has flow rate=10; tunnels lead to valves YW, JT, NN, TK
Valve KQ has flow rate=17; tunnels lead to valves VM, YW, CY
Valve NR has flow rate=0; tunnels lead to valves FK, LR
Valve MN has flow rate=0; tunnels lead to valves OG, XX
Valve YY has flow rate=0; tunnels lead to valves LR, LM
Valve OK has flow rate=0; tunnels lead to valves CY, IN
Valve DK has flow rate=20; tunnels lead to valves FA, RG, IX
Valve CY has flow rate=0; tunnels lead to valves KQ, OK
Valve PR has flow rate=0; tunnels lead to valves DX, LR
Valve DE has flow rate=0; tunnels lead to valves ST, EL
Valve TJ has flow rate=0; tunnels lead to valves WC, HU
Valve NS has flow rate=0; tunnels lead to valves WU, ST
Valve PE has flow rate=0; tunnels lead to valves TM, XO
Valve DU has flow rate=0; tunnels lead to valves IN, IC
Valve DX has flow rate=0; tunnels lead to valves TM, PR
Valve EQ has flow rate=0; tunnels lead to valves AA, GP
Valve AA has flow rate=0; tunnels lead to valves JT, EZ, HZ, DW, EQ
Valve WB has flow rate=0; tunnels lead to valves TM, XX
Valve PF has flow rate=23; tunnels lead to valves BP, WU
Valve FJ has flow rate=19; tunnels lead to valves DO, TY, NN, PS
Valve GP has flow rate=0; tunnels lead to valves XX, EQ
Valve FK has flow rate=4; tunnels lead to valves DW, XO, OG, IC, NR
Valve DO has flow rate=0; tunnels lead to valves XU, FJ
Valve XO has flow rate=0; tunnels lead to valves FK, PE
Valve PS has flow rate=0; tunnels lead to valves RG, FJ
Valve MD has flow rate=25; tunnel leads to valve BP
Valve EZ has flow rate=0; tunnels lead to valves CH, AA
Valve GS has flow rate=0; tunnels lead to valves IN, FR
Valve XU has flow rate=0; tunnels lead to valves DO, ST
Valve WU has flow rate=0; tunnels lead to valves PF, NS
Valve YW has flow rate=0; tunnels lead to valves OV, KQ
Valve HZ has flow rate=0; tunnels lead to valves LR, AA
Valve TY has flow rate=0; tunnels lead to valves FJ, EL
Valve BP has flow rate=0; tunnels lead to valves MD, PF
Valve EL has flow rate=18; tunnels lead to valves DE, TY
Valve UX has flow rate=0; tunnels lead to valves FA, ST
Valve FA has flow rate=0; tunnels lead to valves UX, DK
Valve NN has flow rate=0; tunnels lead to valves OV, FJ
Valve LM has flow rate=0; tunnels lead to valves XX, YY";
