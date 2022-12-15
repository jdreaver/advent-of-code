use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::preceded,
    IResult,
};

fn main() {
    // let readings = parse_input(_EXAMPLE);
    // let row = 10;

    let readings = parse_input(INPUT);
    let row = 2000000;

    println!("part 1: {}", part1(&readings, row));
}

fn part1(readings: &[SensorReading], row: i32) -> usize {
    // Find bounds for beacons
    let first_reading = readings.iter().next().expect("empty covering set");
    let mut min_x = first_reading.beacon_x;
    let mut max_x = first_reading.beacon_y;
    for reading in readings.iter() {
        min_x = std::cmp::min(min_x, reading.beacon_x);
        max_x = std::cmp::max(max_x, reading.beacon_x);
    }

    // Iterate over the row in question
    (min_x..=max_x)
        .filter(|&x| readings.iter().any(|reading| is_point_impossible(reading, x, row)))
        .count()
}

fn is_point_impossible(reading: &SensorReading, x: i32, y: i32) -> bool {
    if x == reading.beacon_x && y == reading.beacon_y {
        return false;
    }

    let sensor_distance =
        (reading.beacon_x - reading.sensor_x).abs() + (reading.beacon_y - reading.sensor_y).abs();
    let point_distance = (x - reading.sensor_x).abs() + (y - reading.sensor_y).abs();

    point_distance <= sensor_distance
}

// fn part1(readings: &[SensorReading], row: i32) -> usize {
//     let mut cover_sets = readings.iter().map(|reading| fill_in_covered(reading));
//     let mut covering = cover_sets.next().unwrap_or_default();
//     cover_sets.for_each(|set| {
//         covering.extend(&set);
//     });
//     println!("done with unions");

//     let &(first_x, _) = covering.iter().next().expect("empty covering set");
//     let mut min_x = first_x;
//     let mut max_x = first_x;
//     // let mut min_y = first_y;
//     // let mut max_y = first_y;
//     for &(x, _) in covering.iter() {
//         min_x = std::cmp::min(min_x, x);
//         max_x = std::cmp::max(max_x, x);
//         // min_y = std::cmp::min(min_y, y);
//         // max_y = std::cmp::max(max_y, y);
//     }

//     println!("starting iter");
//     (min_x..=max_x)
//         .filter(|&x| covering.contains(&(x, row)))
//         .count()
// }

// type Point = (i32, i32);

// fn fill_in_covered(reading: &SensorReading) -> HashSet<Point> {
//     println!("doing reading {:?}", reading);
//     let max_distance =
//         (reading.beacon_x - reading.sensor_x).abs() + (reading.beacon_y - reading.sensor_y).abs();

//     let mut covered = HashSet::new();
//     covered.insert((reading.sensor_x, reading.sensor_y));

//     for distance in 1..=max_distance {
//         for i in 0..distance {
//             // Start vertical, go down and to the right
//             covered.insert((reading.sensor_x + i, reading.sensor_y + distance - i));

//             // Start right, go down and to the left
//             covered.insert((reading.sensor_x + distance - i, reading.sensor_y - i));

//             // Start down, go up and to the left
//             covered.insert((reading.sensor_x - i, reading.sensor_y - distance + i));

//             // Start left, go up and to the right
//             covered.insert((reading.sensor_x - distance + i, reading.sensor_y + i));
//         }
//     }

//     // Actual beacon doesn't count
//     covered.remove(&(reading.beacon_x, reading.beacon_y));

//     covered
// }

#[derive(Debug)]
struct SensorReading {
    sensor_x: i32,
    sensor_y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

fn parse_input(input: &str) -> Vec<SensorReading> {
    input
        .lines()
        .map(|line| {
            let (remaining, reading) = parse_input_line(line).expect("no parse input line");
            assert_eq!(remaining, "", "extra input left in line: {}", line);
            reading
        })
        .collect()
}

fn parse_input_line(input: &str) -> IResult<&str, SensorReading> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor_x) = parse_i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sensor_y) = parse_i32(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = parse_i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = parse_i32(input)?;
    Ok((
        input,
        SensorReading {
            sensor_x,
            sensor_y,
            beacon_x,
            beacon_y,
        },
    ))
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.parse()
    })(input)?;

    Ok((i, number))
}

const _EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

const INPUT: &str = "Sensor at x=3729579, y=1453415: closest beacon is at x=4078883, y=2522671
Sensor at x=3662668, y=2749205: closest beacon is at x=4078883, y=2522671
Sensor at x=257356, y=175834: closest beacon is at x=1207332, y=429175
Sensor at x=2502777, y=3970934: closest beacon is at x=3102959, y=3443573
Sensor at x=24076, y=2510696: closest beacon is at x=274522, y=2000000
Sensor at x=3163363, y=3448163: closest beacon is at x=3102959, y=3443573
Sensor at x=1011369, y=447686: closest beacon is at x=1207332, y=429175
Sensor at x=3954188, y=3117617: closest beacon is at x=4078883, y=2522671
Sensor at x=3480746, y=3150039: closest beacon is at x=3301559, y=3383795
Sensor at x=2999116, y=3137910: closest beacon is at x=3102959, y=3443573
Sensor at x=3546198, y=462510: closest beacon is at x=3283798, y=-405749
Sensor at x=650838, y=1255586: closest beacon is at x=274522, y=2000000
Sensor at x=3231242, y=3342921: closest beacon is at x=3301559, y=3383795
Sensor at x=1337998, y=31701: closest beacon is at x=1207332, y=429175
Sensor at x=1184009, y=3259703: closest beacon is at x=2677313, y=2951659
Sensor at x=212559, y=1737114: closest beacon is at x=274522, y=2000000
Sensor at x=161020, y=2251470: closest beacon is at x=274522, y=2000000
Sensor at x=3744187, y=3722432: closest beacon is at x=3301559, y=3383795
Sensor at x=2318112, y=2254019: closest beacon is at x=2677313, y=2951659
Sensor at x=2554810, y=56579: closest beacon is at x=3283798, y=-405749
Sensor at x=1240184, y=897870: closest beacon is at x=1207332, y=429175
Sensor at x=2971747, y=2662873: closest beacon is at x=2677313, y=2951659
Sensor at x=3213584, y=3463821: closest beacon is at x=3102959, y=3443573
Sensor at x=37652, y=3969055: closest beacon is at x=-615866, y=3091738
Sensor at x=1804153, y=1170987: closest beacon is at x=1207332, y=429175";
