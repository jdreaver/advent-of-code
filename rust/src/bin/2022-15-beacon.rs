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
    // let part2_max = 20;

    let readings = parse_input(INPUT);
    let row = 2000000;
    let part2_max = 4000000;

    println!("part 1: {}", part1(&readings, row));
    println!("part 2: {}", part2(&readings, part2_max));
}

fn part1(readings: &[SensorReading], row: i32) -> usize {
    // Find max bounds
    let first_reading = readings.iter().next().expect("empty covering set");
    let mut min_x = first_reading.beacon.0;
    let mut max_x = first_reading.beacon.1;
    for reading in readings.iter() {
        min_x = std::cmp::min(min_x, reading.sensor.0);
        max_x = std::cmp::max(max_x, reading.sensor.1);
    }

    let max_distance = manhattan_distance(first_reading.beacon, first_reading.sensor);
    min_x -= max_distance;
    max_x += max_distance;

    // Iterate over the row in question
    (min_x..=max_x)
        .filter(|&x| is_point_impossible(readings, x, row))
        .count()
}

fn is_point_impossible(readings: &[SensorReading], x: i32, y: i32) -> bool {
    readings
        .iter()
        .any(|reading| (x, y) != reading.beacon && in_sensor_range(reading, x, y))
}

fn in_sensor_range(reading: &SensorReading, x: i32, y: i32) -> bool {
    let sensor_distance = manhattan_distance(reading.sensor, reading.beacon);
    let point_distance = manhattan_distance(reading.sensor, (x, y));

    point_distance <= sensor_distance
}

fn part2(readings: &[SensorReading], max_bound: i32) -> i64 {
    let known_beacons = readings
        .iter()
        .map(|reading| reading.beacon)
        .collect::<HashSet<Point>>();

    // Since there is only a single point for the solution, it must lie just
    // outside the boundary of a scanner.
    let (x, y) = readings
        .iter()
        .flat_map(reading_outside_boundary)
        .filter(|&(x, y)| x >= 0 && x <= max_bound && y >= 0 && y <= max_bound)
        .filter(|point| !known_beacons.contains(point))
        .find(|&(x, y)| readings.iter().all(|reading| !in_sensor_range(reading, x, y)))
        .expect("no solution found");

    (x as i64) * 4000000 + (y as i64)
}

fn reading_outside_boundary(reading: &SensorReading) -> Vec<Point> {
    let distance = manhattan_distance(reading.sensor, reading.beacon) + 1;
    (0..distance)
        .flat_map(|i| {
            vec![
                // Start vertical, go down and to the right
                (reading.sensor.0 + i, reading.sensor.1 + distance - i),
                // Start right, go down and to the left
                (reading.sensor.0 + distance - i, reading.sensor.1 - i),
                // Start down, go up and to the left
                (reading.sensor.0 - i, reading.sensor.1 - distance + i),
                // Start left, go up and to the right
                (reading.sensor.0 - distance + i, reading.sensor.1 + i),
            ]
        })
        .collect()
}

type Point = (i32, i32);

fn manhattan_distance(a: Point, b: Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[derive(Debug)]
struct SensorReading {
    sensor: Point,
    beacon: Point,
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
            sensor: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
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
