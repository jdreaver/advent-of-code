use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = parse_input(INPUT);
    println!("part1 {:#?}", min_or_max_length(&input, true));
    println!("part2 {:#?}", min_or_max_length(&input, false));
}

#[derive(Debug, PartialEq)]
struct CityDistance {
    start: String,
    end: String,
    distance: u64,
}

fn min_or_max_length(distances: &[CityDistance], do_min: bool) -> u64 {
    // Collect all cities and inter-city distances
    let mut cities: HashSet<String> = HashSet::new();
    let mut distance_map: HashMap<(&String, &String), u64> = HashMap::new();
    for CityDistance { start, end, distance } in distances {
        cities.insert(start.clone());
        cities.insert(end.clone());
        distance_map.insert((start, end), *distance);
        distance_map.insert((end, start), *distance);
    }

    // Iterate over all permutations of cities
    let lengths = cities
        .iter()
        .permutations(cities.len())
        .flat_map(|path| path_length(&distance_map, &path));
    if do_min {
        lengths.min().expect("no path distances")
    } else {
        lengths.max().expect("no path distances")
    }
}

fn path_length(distances: &HashMap<(&String, &String), u64>, path: &[&String]) -> Option<u64> {
    let mut path_len = 0;
    for (city1, city2) in path.iter().tuple_windows() {
        path_len += distances.get(&(city1, city2))?;
    }
    Some(path_len)
}

fn parse_input(input: &str) -> Vec<CityDistance> {
    input
        .lines()
        .map(parse_city_distance)
        .collect()
}

fn parse_city_distance(line: &str) -> CityDistance {
    let (start, rest) = line.split_once(" to ").expect("failed splitting on ' to '");
    let (end, distance_str) = rest.split_once(" = ").expect("failed splitting on ' = '");
    let distance = distance_str.parse::<u64>().expect("failed parsing distance");
    CityDistance {
        start: start.to_string(),
        end: end.to_string(),
        distance,
    }
}

const _TEST_INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

const INPUT: &str = "Faerun to Norrath = 129
Faerun to Tristram = 58
Faerun to AlphaCentauri = 13
Faerun to Arbre = 24
Faerun to Snowdin = 60
Faerun to Tambi = 71
Faerun to Straylight = 67
Norrath to Tristram = 142
Norrath to AlphaCentauri = 15
Norrath to Arbre = 135
Norrath to Snowdin = 75
Norrath to Tambi = 82
Norrath to Straylight = 54
Tristram to AlphaCentauri = 118
Tristram to Arbre = 122
Tristram to Snowdin = 103
Tristram to Tambi = 49
Tristram to Straylight = 97
AlphaCentauri to Arbre = 116
AlphaCentauri to Snowdin = 12
AlphaCentauri to Tambi = 18
AlphaCentauri to Straylight = 91
Arbre to Snowdin = 129
Arbre to Tambi = 53
Arbre to Straylight = 40
Snowdin to Tambi = 15
Snowdin to Straylight = 99
Tambi to Straylight = 70";
