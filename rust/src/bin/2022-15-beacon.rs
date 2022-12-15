use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::preceded,
    IResult,
};

fn main() {
    let readings = parse_input(_EXAMPLE);
    println!("{:?}", readings);
}

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
