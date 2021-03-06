fn main() {
    assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
    assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
    assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
    assert_eq!(get_seat_id("BBFFBBFRLL"), 820);

    let seat_ids = INPUT.lines().map(get_seat_id).collect::<Vec<u32>>();
    let max_seat_id = seat_ids.iter().max().expect("couldn't get max for part 1");
    println!("part1: {}", max_seat_id);

    println!("part2: {}", find_my_seat(&seat_ids));
}

fn get_seat_id(input: &str) -> u32 {
    SeatLocation::from_partition(&SeatLocationPartition::from_string(input)).id()
}

fn find_my_seat(seat_ids: &[u32]) -> usize {
    // It's a completely full flight, so your seat should be the only
    // missing boarding pass in your list. However, there's a catch:
    // some of the seats at the very front and back of the plane don't
    // exist on this aircraft, so they'll be missing from your list as
    // well.
    //
    // Your seat wasn't at the very front or back, though; the seats
    // with IDs +1 and -1 from yours will be in your list.

    let max_id = *seat_ids.iter().max().expect("max seat id") as usize;
    let mut seat_in_list: Vec<bool> = vec![false; max_id + 1];
    for seat_id in seat_ids.iter() {
        seat_in_list[*seat_id as usize] = true;
    }

    for i in 1..(max_id - 1) {
        if !seat_in_list[i] && seat_in_list[i - 1] && seat_in_list[i + 1] {
            return i;
        }
    }

    0
}

#[derive(Debug)]
enum BinaryPartition {
    Upper,
    Lower,
}

fn partition_location(partitions: &[BinaryPartition]) -> u32 {
    let mut min = 0;
    let mut size = 2_u32.pow(partitions.len() as u32);

    for partition in partitions {
        size /= 2;
        match partition {
            BinaryPartition::Upper => {
                min += size;
            }
            BinaryPartition::Lower => {}
        }
    }

    assert_eq!(size, 1);

    min
}

#[derive(Debug)]
struct SeatLocation {
    row: u32,
    column: u32,
}

impl SeatLocation {
    fn from_partition(partition: &SeatLocationPartition) -> Self {
        SeatLocation {
            row: partition_location(&partition.row),
            column: partition_location(&partition.column),
        }
    }

    fn id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

#[derive(Debug)]
struct SeatLocationPartition {
    row: Vec<BinaryPartition>,
    column: Vec<BinaryPartition>,
}

impl SeatLocationPartition {
    fn from_string(input: &str) -> Self {
        let mut location = SeatLocationPartition {
            row: Vec::new(),
            column: Vec::new(),
        };
        let chars = input.chars().collect::<Vec<char>>();

        for i in 0..7 {
            match chars[i] {
                'F' => location.row.push(BinaryPartition::Lower),
                'B' => location.row.push(BinaryPartition::Upper),
                _ => panic!("unknown char {}, expected F or B", chars[i]),
            }
        }
        for i in 7..10 {
            match chars[i] {
                'L' => location.column.push(BinaryPartition::Lower),
                'R' => location.column.push(BinaryPartition::Upper),
                _ => panic!("unknown char {}, expected L or R", chars[i]),
            }
        }

        location
    }
}

const INPUT: &str = "BFBBBBBLLR
BBFFBBFRRL
FBFBFFFRRL
BBFFFBFRRL
BFFBFBFRLL
FFBBBFBLRL
BFFFBFFLLR
FBFFFFBLRR
FBFFBBBRRR
BFFBFFFLLR
BFFFFFBLLL
FFBBFFBLRR
BFFFFFBRLR
FBFBFBFLLL
FFBFBFFLLL
BBFFFFFLLL
FFFBBBFLLL
BBFFBFBLLR
FFBFBFBRRR
BFFBFFFRRR
BBFFBFBRLR
FFFBFBFLRL
BBFFBBBLRR
FBBFFFFLLR
BBBFBBFLLL
BFFFFBBRLL
FBBFBFBRRL
FFBBBFFRRR
BBFFFFBRRR
FBBBBFFLLR
BFFFFFBLRR
FFBBFBFLLR
BFFBFBBLLL
BBFBBBBLLL
FBFBBBFRLL
FFBFFFBLRL
FFFFBBBLRL
BBBBFBFRRR
FFBBFBBRLL
FFFBBBBRRL
FFBFFFBRLR
BFBFBFBLLL
FFFBFFBLLL
BBFBFBBLRL
BBBBBFFLLL
FBBBBFFRRR
FBFFFFBLLL
FFFFBBBRRL
FBFBBBFRRR
FBBFFBFLRL
BFBFFBFLLR
FFBBBFBLLL
BFBFFFFLLL
BBBFFFFLLL
BFBBFBFRLR
BFBBBBFLRL
BBBFBFBLRL
BFFFBFBLRL
BFBBBBBRLL
FBBBFFBRLL
BFBFFBBRRL
BFFBBBFLLR
BFFBFFBRRR
FBFBFBFRRL
FBFBFBFLRL
BBBFFFBRRR
FBFFBBFRRR
FFBBFFFRLL
BFBFBBBLLR
BFFFFBFLLR
FFBBBBFRLR
FFFBFFFRRR
BFBBFFFRLL
BBBFBFFLRL
BFBBFBFLRR
BBBBFBBLRL
FBFBFBFLRR
BBFBFBBLLL
BFBBBFBLRL
BFBBFBFLRL
FFFBBFBLLL
BBBFBFBLLR
FBBBBBFLLR
FFBBBFBRLL
BBBBFBBRRL
FBFFFBBRLL
BBFBBFFRRL
FBFFBFBLLL
BFFBFBBRRL
BFFFBFFRLR
BBFBFBFRRR
FBBFBFFRRR
BBBFBFBLRR
FFBFFBFRLR
FFBFBBBLRL
FFFFBBFRRL
FFBBBFBRLR
FBFBFBBLLL
FBBBFBFRLL
BBBBFBBLLR
BFBFBBFRLR
FBBFBBFLRR
BBBBFBFLRL
FBFBFBFRLL
FFBFBFBLLR
FBFFBBFRRL
BFFFFFFRLR
FFFFBFFRRL
BBFFFBFLRR
FBBFFBFRRL
FFFFBFBLLL
FFBBFFFRRL
BBFBFFFRLL
BFFFFBFRLR
FBFBFBBRRL
FBBFBBFRLR
BFBFFFBRRL
FFFBFFBRLR
BFBFFBFLRL
FBBFBFFRRL
FFBFFFBLLL
FBFFFBFRRR
FBFFFBBRRL
FBFFBFBRLL
BFBFFFFRLL
FFFBBBFLRR
BBFFFFFLRR
BFBBFBBLLR
BFBFFBFLLL
FBBBBFBLLR
BFBBFFFLLR
BFBBFFFRRL
BFFBBFFLLR
BBBFFFFRRL
BBBFBBBLLR
FFFBFBBRLL
FFBFFBBLRR
FFBFBBFLLL
BBBFFBFRLL
BBBFBFFLRR
FBFBFFBRLR
FBBFFBBLRL
BBFBBFFLLR
BBFBBFBRLR
BFBFFBBRLR
BBBBFFBRLL
FFFBBBFRLL
FBBFFFFRLR
BFBBBBFRRR
BBBBFFBLRR
FBBBBBFLRR
FBBBFFBLRL
BFFBBBFRRR
BBBBFBFRLL
BFBBBFBLRR
FFBFFFFLLL
FBFFFFBRLL
FFBFFBBRLR
BFFBBFFLLL
FFBFBFBLRL
FFBFBFFRRR
BBBFBBFLRR
BBBFBBBLLL
BFFFFFFLRL
FBBBBBBRLR
FFFBFBFLRR
FFBBFFBRRL
FBBFBBBRLR
BBBFBBFLLR
FFBFFBFLRR
BFBBFBBRRL
FBFFFBBLRL
BFFBBFBLRL
BBFFBFFLLL
FBBBFBFRLR
BBBBFFFRLL
FFBFBFBLRR
FBBFFFBRRL
FFBBBFFLLL
BBFFFBBLLR
BFFFFBFRRL
FBFFFFBRRL
BFBBFFFRLR
FFFBBBBLLL
FBFBFBBRLR
FFFBFBBRLR
FBFBFFFLRR
BFFFFFFRRR
FFFFBBBRRR
BBFBFFFRRL
FFFBFBBLLL
BFBBFBFLLR
BBFBBBBRRL
BBFFBBFRRR
BBBFFBBLRL
FBFBFFFLLR
BFFFBBBRRR
FFBFFFFLLR
FBBBFBFLLL
FBBBFBBLRR
BBBFBFBRRR
FBBBFBFRRL
BBBBFFBRLR
FBFBBBBLLR
FFFFBBFRLR
BBFBBBFLLL
BBFBBBBLRR
FFFFBFBRRR
FBFFFBFLRR
BFFFBBFLRR
BBBFBFFLLR
BFFBBBBRLR
FBFBBFBLLR
BFBBBFFLRR
BFFBBFFRRR
FFFBFFFRLL
FFBFBBBRRR
BBBFFFBLLR
BBFFFFFLLR
FBBBBFBLRR
FBFBBFFRRL
BFBFBFFRRL
BFFFFBFRRR
FFBFFBBLLL
BFBFBFBRRR
BFFFFFFLLR
FFFBBFBRRL
BFBBBFBRRL
BBFFFBFRLL
BFBBBBBLRL
FBFBBFFLRR
FFBBBFFRLR
FBBBFBBRRR
FFBBBFFRRL
FFBBBBFLLR
BBFBBFBRRL
BBFBBBFRLL
BBFFFFFLRL
BBBFFFBLLL
BBFFFFFRRR
FFBFBBBLRR
FFBBBFFLRR
BFFBFBFRLR
FFBBBFBRRR
FBBBBFBLRL
BBBFBBBRLL
FBFBFFBRRL
BFFFFBBRRL
BBFFBBFRLR
FFBFBFFRLR
BFFFBFBRRR
BBBFBBFRRL
FFFBFFFLRL
FBBFFBFRRR
FFBBBBBLLL
FFBFFBFLRL
FBFBFBBLRR
FFFBFFBRLL
BBBFFFBRRL
BFBFBBBRLL
BBBFFFFLRR
BBFFBFFLLR
FBBFFBFLRR
FBBBFFBLRR
BFBFFBBLLL
FBFFFBBRLR
FBBFBBBLLR
BFBBBBBRLR
FBBBFFFLLL
BFFFBBFLLL
BFFFFBBRLR
FBBFBFBRLL
FBFFBFBLRL
BBBFFFFRLL
FBFFFFFRLL
FBBBFBFLRL
FBFBBFBRRL
FFBFBFFLLR
BBBFFBBLLL
FFBFBBFLLR
BBBFBBFRLL
BFFBFBBLRR
FBBFBBFRRL
FBFFBFFLLL
BFBBFBFLLL
FFBBFFBRLL
BFFFFFFLLL
BFFFFFFRLL
BFBFFFFLRR
FFBBFFFLRR
BBFFBBBRLR
BFFFFBFLRR
FFBFBFFLRR
BBBBBFFLRL
BBFBBBBRLR
BFFBFFBRLL
BFBFBBFLRL
FBFBBFBLLL
BFFBBFFRLR
FFFFBFBRLR
BFBFBFBRLL
BFBBFFBLRR
BBBBBFFRLL
BBFFFBBRLL
BFBFBBBLLL
FBBBFFFLRR
FBBBBBFRRL
BFFBBFBRRL
FFBBFFBLLL
FBFFFBFRLL
BBBFFFBRLL
FFFBBFFLLL
BBFBFBBLLR
FFFBFFBLRL
BFFBBBBLLL
BBFFBFFLRR
FFBBFBFLRR
BBBBFBFRRL
BBFBFBBRRR
BBFFFFBLLR
BBBBFFBLLR
BBBFFBFLRR
FFBFFBBLRL
BBFBBBFRLR
FBFFBFFLRL
FFFFBFBLRL
BFBFFFBLRL
FBBBFBFLRR
BFFFBBBLLL
FBBBFFFRRR
BFBFBFFLLR
BFFBFBFLLR
FFFBBBFRRL
BBBFFBFLLR
FFBFBFBLLL
BFBBBFFRLR
BBBFFBFRRL
FBFBFFFRLR
BFBFBBFRRL
FBFFFFFLRR
BFBFFBBLLR
BBFBFBBRLR
FBBBFFFLRL
BBBBFBBRRR
FFBBBBFLLL
FBFFFBBLLR
FFBBBBBRRR
BFFFBBBLRR
BBFBBBBRLL
BBFBFBFLRR
BFBBFFBLLR
FBBFFBBRLL
FBBBBFFRLL
FBFBFFBLLR
FFBFBBBRRL
FFFBBFBRLL
BFFBBFBLLR
BBBBBFFRRL
BFBBFFBRRL
BFFBFFBRLR
FFBFBBFRRL
FFFFBBFLRL
BBFFFFBLRL
BFBFFFBRLL
BFBFFFFRRR
BBBFFBBRRL
FFFFBBFLLL
BBBFFFFLRL
BBBFBFFLLL
FBFBFFBRLL
FBBFFFBLRL
BFBFBFFLLL
BBBFFFBLRR
FBFBFBFRLR
BFBFBFFLRL
BFFFBBBRRL
BFBFFBBRLL
FFBFFFBRLL
FFBFFBFLLR
FFFBFFBRRL
FFBBFFFLRL
BFBFFFFRRL
FBFFFFFRRR
BBFBBFBLLR
FFBFBFBRLR
BFFFBFFRRR
FBBBBFBLLL
BBFFBBBRRR
BBFBBBBRRR
BBBFBBBLRR
BBFBBFBRLL
BFBFBFFLRR
FBFFFBFLRL
FFFFBFFRRR
BFFBFFFLRR
FBFFBBBRLL
BFFFBFBLLL
FBBFBFBLLR
FFBFFBBRRR
BFBFBBFLRR
BBFFFFBLLL
FBBFFFFRRR
FBFBBBFRRL
FBBFFFBLLR
FBFBBFBRLR
BBFFBBBLLR
FFFBBFFLRL
BBFFBFBRRR
BFBFFFFLRL
FFFFBFBRLL
FBFFFBFRRL
BBBFBBBLRL
FFBBBFFRLL
BBFBFFBRRL
BFFBFFBLLL
BBFBBFBRRR
FBBFBBFLLR
BFFFFFFLRR
FBBBBBBRRR
BBFBBBFRRL
FFFBBBFLLR
FBBBBFFLRL
FBFFBFFRLL
BFBFBFBRLR
FBBBFFFRLL
BBFFFBBRRR
BBFBFBBLRR
BFFFFFBRLL
FBFBFFFLLL
FBFFBFBLLR
BFFBFFBRRL
BFBBFFFLRR
BFBBBFBLLR
FFFBBBBRLR
FFBFFBFRRL
FFBBBFBLLR
BBFBFFBRRR
FFBBBBBRLR
BBFFBBBLLL
BFBBFFBLLL
BFBFBBFRRR
BBBBFFFLLL
FBBFFBBRRL
FFBBBFBRRL
BFBFBBBRRR
FBFBBFFLLL
FFBBFFFLLR
FBBBBBFLLL
FFBFBFFLRL
FFBFFBFRRR
FBBBFBBRLR
BFFFFBFLRL
FFFBFFFLLR
BBFBFFBLLR
FFBBFFBLRL
FFFBFBBLRL
BFFFBBBRLR
BBFFFFBRRL
FBBFFBBLRR
FBFFFFFLRL
BBFFBFFRLR
BBFBFFFRLR
FFFBFBFRLR
BFFFBBFRRL
BBBFFBFLLL
BBFBBBBLRL
BFFFFFBLLR
BFFFFBBLLR
BBBFFBFRRR
BBFFFFFRRL
BBFFFFBRLR
FFBBBFFLRL
BFFFBBBRLL
BFFFBBFRLL
FBFBBBFRLR
BBFBFBFRRL
FBFFBFBRLR
FBFFBFFRRR
FBFFFBBLRR
BFFFBBFLRL
FBFBFFBLRL
FBBBBBBRRL
BFFFBFBRLR
FBBBBFBRLL
BBFFBBFLLR
BBFBBFFLRR
FBFFFBFLLR
FBBBFBBLLL
FFBFFFBLRR
BBFBFFBLRR
FFBFFBBRLL
BFFBBFFRLL
FBFFFFBRRR
FFFBFBBRRL
BBBBFBFLLR
BBFBFFFLRL
BFBFFFFRLR
BBFBFFFRRR
BBBFBBBRRL
BFFBBBBLRR
BFFFBBBLRL
BBFFFBBRLR
FBBBFFBRLR
FBBBFFBLLL
FBBBBBBLRL
BBBFFFBLRL
FBFBFFFLRL
BFBFFBFRLL
BBFFFFBRLL
BBFFFFFRLL
FFFBFFFRRL
BBFFBBFLRR
BBBBFFFRRL
BFFBBBBLLR
BBBBFFFRLR
BBBFFFBRLR
FBBFFBBRLR
BFFBBFBLLL
BBBFBBFLRL
BFBFBBFLLR
BBFBBFFLRL
BFFBBBFRRL
FFFFBBFLRR
FBFFBBBLLR
BFBFBFBLLR
FBBBFFBRRR
FBBBBBFRLR
BBFBFFBRLR
BBFFBFFRLL
FFFBFFBLLR
BBFFBFBLRR
FBFFFBBRRR
BFFBBBBRRR
FBFFBBBRRL
BBFBFBFLLR
FBFBBFFLRL
BBBBFFFRRR
BBFBFFFLLL
BFFFBFBLLR
FBBBBFFLRR
FBFFBBFLLL
FFFFBFBRRL
BBFBFBFLLL
BFBBFFBLRL
BFFFBFBLRR
BBBBFFBRRR
FBFBBFBRRR
FBBFBFBRLR
FBBFBFFLRR
FBFBBBBLRR
FBBFBFFRLR
FBBBBBFLRL
FBBFBFFRLL
FFBBFFBRRR
BFBFBFBLRR
FBBFBBBLRL
FBBBFBFRRR
BFFBFBFRRL
FBBFFFFLRL
BFBBBBFRLR
FFBFBBBRLL
BFFBBBBRRL
BFBBFBBLLL
FBFBFFFRRR
BFBFBFBLRL
FBBFFBFLLR
FFFBFBBRRR
FBBBBFFLLL
FBFBBBBLRL
FBBFBFBLRL
FFBBFBBRRR
FBFBFFBLRR
FBFBFBBLLR
FFFBFBFRLL
FFFFBBFRRR
BFBBBBBLLL
BBFBBBBLLR
FBBFFFFRRL
FBBFBFBLLL
BFFFBBFLLR
FBBBFFBLLR
FFBFFBFLLL
BBBFFBBLLR
BFBFFFFLLR
FBFBBBFLRR
FFBBBBFRRL
FFFFBBBLRR
FFBFFFFRRL
BFBBFFFLRL
BBBFBFBLLL
FBFBBFBLRR
BBFBBFBLLL
BFFBFBBRLR
BFBBFBBRRR
FFBBFBFRLL
BFFBFBFLLL
BFFBBFBLRR
FBBBBFBRRL
BFBFBBBLRR
FBBBBBBLLL
BFFFBBBLLR
BBFBFFFLLR
BBBFFBBRLL
BFFBBBFLRR
BFFBFBFRRR
BBBBBFFRLR
BFFBFFFRLR
FBFFFFFLLL
BFFFBFBRLL
FBBFBBFLLL
FBBFBFBRRR
FBFFBBFLLR
FBBBFBFLLR
FFFBFFFLRR
FFBBBBFRRR
BBBBFBFLRR
FBFBFFBRRR
BFFBFFBLLR
FFBBFBBRLR
FBFBFBBRRR
FFBBBBBLRL
FBBBBBFRRR
FFFBFBFRRL
FFBFBFBRRL
BFBBBFBRLR
BBBFBFFRRR
BFFBBFBRLR
BFFBBBBRLL
FBBBBFFRRL
BBBFBBBRLR
BFFFFFFRRL
FFFBBFFLRR
BBFBFFBLRL
BFFBFBFLRR
BBBFFBBLRR
BFBFFBFRRR
BBBBFFBLLL
FBFFBBBLRR
BBFFBBFLLL
FBFBBBBRRR
BBFFBFBLLL
BBBBFFFLRR
FBFFBBBRLR
BFBFFFBLRR
BBFBBFFLLL
BFFBBFFRRL
BFFFBFFRRL
BBFFFBBLRR
FFBFFFFLRL
FFBFBBFRLL
FBFBFFFRLL
BBFFFBFRLR
FFFBBFBLRR
BFBBBFBLLL
BBFBFBFRLL
BFBFBBFRLL
BFFFFFBRRL
FFBFBBBRLR
BFBBFFBRLR
BFBFBFFRLR
FBBBBBBLRR
FFBBBBFLRL
FFFFBFBLRR
FFBFFFBRRR
FFBFBBFRLR
BBBFFBFLRL
FBBBBFFRLR
BFFBBFBRLL
BFFBBBFRLR
FFFBFFBRRR
BBBFBBBRRR
FBFFBFFRRL
FBFFFBFLLL
BFBBBFFRRL
BBFBFBBRLL
BBFBFBBRRL
FFFBFBFRRR
BFBBFBBLRR
FFFBBBFRRR
BFFBFBBLRL
BFFBFFBLRR
BFBBBFFRRR
FFFBBBFLRL
FFBBFBFRRR
FFBFBFBRLL
FFBFFFBLLR
FBBBBBBRLL
FFFFBFBLLR
FFBBFFFRRR
FFFBFFFLLL
FFBBFBBLLR
FFBBBBFLRR
FBBFFBBLLL
BFBBFBBRLR
FFFBFBBLLR
BBFFBFBLRL
FFBFBBFRRR
BBBBFBBRLR
BBBFBBFRLR
BBFFBFFRRR
FFBBBBBLLR
FBFFBFBLRR
FBBBBFBRLR
FFFFBBFLLR
BFBFBBFLLL
FBBFBFBLRR
FBBFFBFRLL
BBFFFFFRLR
FFBBFBFLRL
BFBFBBBRLR
FBBFBBFRRR
FBFBFBBLRL
BBBFFBBRRR
FBBFFBBRRR
FBFFFFBRLR
FBBFBBFLRL
FFFBBFBRRR
FBFBBBBRLR
FBBFFBBLLR
FFFBBBBRRR
BBBBBFFLLR
FFFFBBBLLL
BFFBBBFRLL
FBFBBFBLRL
BBBBFBFRLR
BBFBFFBLLL
BFBFFBFLRR
BFFFFFBLRL
BBBFBBFRRR
FFBBBBBLRR
FBFFFFFLLR
FBFBBFFRLL
BFBBBBBRRL
FBFBFBBRLL
FBBFBBBRRL
FBFFBBBLRL
BFFFBFFLLL
BFBBBBBRRR
FFFBBFFRRR
BBFBBBFLRL
FFFFBBFRLL
FBFFFFFRLR
FFFFBBBRLR
FBFFBBFRLR
FFFFBBBRLL
BBFFBBBLRL
FFBFBBFLRR
BFBBBBBLRR
BFFFFBBLLL
FBFBBBBRRL
BBBFBFFRRL
BFBBFBBLRL
BFBBBBFRRL
FBFFFBBLLL
FFBBFBBLRL
BBFBFFBRLL
BBBFFFFRRR
FFFBBBBLRR
FFFBBFBLRL
FFFBBBBLLR
BFBFBBBRRL
BBBBBFFLRR
BBBBFBBLLL
BBBBFBBLRR
FFFBBBFRLR
FBBBFFFLLR
BFFFBFBRRL
BBFFFBFLLR
BFBBFBFRLL
BFBBBFFLRL
FFBFFBBRRL
FBFBBFFRLR
BFFFFFBRRR
FBFFFFBLLR
FBBFBFFLLR
FBBBFFFRLR
FBFBFBFLLR
FFFBBFBLLR
BFBFFBFRRL
FFFBBFBRLR
FFBBFBBLRR
FBBBFBBLLR
BBBFFFFLLR
BBBBFBFLLL
BBFFFBFLLL
BFFFBFFLRL
BBFBBFFRLR
BBFFBBFLRL
FBBBBFBRRR
FBBFFBFLLL
FFFBFFBLRR
FFBBFFFRLR
FBBFFFBRLR
BBBFBFBRLL
BFBBFFFLLL
FFFBBFFRLL
BBFFBFBRLL
FFBBBBBRRL
FBFFBBBLLL
FBBFFFBRRR
BFFFBFFRLL
FFBBBFFLLR
FBBFBBFRLL
FBBFFFFLRR
FBBBBBFRLL
BBBFFBBRLR
FBFFBBFLRL
FBBFFFBLLL
BFBBBFFRLL
FFFBBFFRLR
BFBFFFBLLL
FFFFBBBLLR
BFFBFFFLLL
BFBBFFBRRR
BFFBFBBLLR
FFBFFBFRLL
FFFBBFFLLR
FBFFFFFRRL
BFFBBFFLRR
FFBBFFBLLR
FBBBFFBRRL
FBBBFBBLRL
BBFFBFFLRL
BBBBFFBRRL
BFBFFBBLRL
BFBBFBFRRR
FFFBBBBRLL
FBBFBBBRRR
BBBBFFFLRL
FBFFBFBRRR
BBBBFFFLLR
FBBFFBFRLR
FFFBBBBLRL
BBFBBFBLRL
BFBBBBFLLR
BBBFBFBRRL
BFBBBBFLLL
BFBFFFBRRR
FFBBFFBRLR
FBBBFFFRRL
BFFFFBBRRR
FBFBBFBRLL
BFBBFFBRLL
FBBFBFFLRL
BFFBBBBLRL
BFFBFBFLRL
FBBBFBBRRL
BFBBBBFLRR
BFBFFBBRRR
BFFFBBFRLR
BFFBFFBLRL
BFFBBFBRRR
BBFFBBBRRL
BBBBFBBRLL
FBBFBBBLRR
FFBFBFFRLL
FFBFFFFRRR
BFFBBFFLRL
BFBBFBBRLL
BFBFFBBLRR
FBFBFBFRRR
BBFFFBFRRR
BBFBBFFRRR
FFBFBBBLLR
FFFBFBFLLL
BBFFFBBLLL
FFBBFBBLLL
BFBBFFFRRR
BFBFBBBLRL
FBFBBBFLRL
BFBFBFBRRL
FFFBFBFLLR
BBFFFFBLRR
BBFBFBFRLR
FFBBFBFLLL
BFBBBFBRLL
BBBFFBFRLR
BFFFFBFLLL
FFBBBBFRLL
BBFBBFFRLL
BFFFBFFLRR
BBFBBBFLLR
FFBFBBFLRL
BBBBFFBLRL
BFBBBFBRRR
FBBFBFFLLL
FBFFBFFRLR
BFFBFFFLRL
FBFFBFFLLR
FBFBBBBRLL
BBFFFBFLRL
BBFBBFBLRR
BFFFFBBLRL
BBFBFBFLRL
FBBFBBBRLL
FBBFFFFRLL
FFBFBBBLLL
FFBFFBBLLR
BFBBBFFLLR
BBBFBFFRLR
BFFBFFFRRL
BBFFBBBRLL
BBFFBBFRLL
FBBBBBBLLR
BFFBFBBRRR
BBFFFBBRRL
FFBFFFFRLL
FFFBFBBLRR
BBFFBFBRRL
BBFFFBBLRL
FBFFFFBLRL
BFFFFBFRLL
BFFBBBFLLL
BFBBFBFRRL
FBBFFFBLRR
FFBFFFFRLR
FBBFFFFLLL
FBFBBFFRRR
FBFBBFFLLR
FFBFFFBRRL
BFBBBBFRLL
BFFBBBFLRL
FBFFBFBRRL
FFBFFFFLRR
FBBBFBBRLL
BBFBBBFRRR
BFFBFFFRLL
FFFBBFFRRL
BFBFFFBLLR
FBFFFBFRLR
FBFBBBFLLL
FFBBFBFRRL
BBBFBFFRLL
FBFFBFFLRR
BBFBBBFLRR
FFBBBFBLRR
FFFBFFFRLR
FBFBFFBLLL
BFBBBFFLLL
BFFFFBBLRR
FBFBBBFLLR
FBFFBBFLRR
FFBBFBBRRL
BBFBFFFLRR
BBFFBFFRRL
FFBBFFFLLL
FFBFBFFRRL
BBBFBFBRLR
BFBFFFBRLR
FBFBBBBLLL
FFBBBBBRLL
FBBFBBBLLL
BFFFBBFRRR
BFBFFBFRLR
BBBFFFFRLR
FBFFBBFRLL
FFBBFBFRLR
BFFBFBBRLL
BFBFBFFRRR
FBBFFFBRLL";
