use std::cmp;
use std::collections::HashMap;

fn main() {
    let seats = parse_input(INPUT);
    println!("part1: {}", stable_occupied_count_part1(&seats));
    println!("part2: {}", stable_occupied_count_part2(&seats));
}

fn stable_occupied_count_part1(seats: &Vec<Vec<Option<Seat>>>) -> usize {
    let mut seats = seats.clone();
    let mut result = 0;
    loop {
        seats = sim_step_part1(&seats);
        // println!("{}\n", print_input(&seats));
        let this_occupied = num_occupied(&seats);
        if this_occupied == result {
            return result;
        }
        result = this_occupied;
    }
}

fn sim_step_part1(seats: &Vec<Vec<Option<Seat>>>) -> Vec<Vec<Option<Seat>>> {
    let mut out = seats.clone().to_vec();
    for (row, line) in seats.iter().enumerate() {
        for (col, seat) in line.iter().enumerate() {
            let mut occupied_neighbors = 0;

            for i in row.saturating_sub(1)..=cmp::min(seats.len() - 1, row + 1) {
                for j in col.saturating_sub(1)..=cmp::min(line.len() - 1, col + 1) {
                    if !(i == row && j == col) && seats[i][j] == Some(Seat::Occupied) {
                        occupied_neighbors += 1;
                    }
                }
            }

            // If a seat is empty (L) and there are no occupied seats
            // adjacent to it, the seat becomes occupied.
            if *seat == Some(Seat::Empty) && occupied_neighbors == 0 {
                out[row][col] = Some(Seat::Occupied);
            }

            // If a seat is occupied (#) and four or more seats
            // adjacent to it are also occupied, the seat becomes
            // empty.
            if *seat == Some(Seat::Occupied) && occupied_neighbors >= 4 {
                out[row][col] = Some(Seat::Empty);
            }
        }
    }
    out
}

// Computes which seats a given seat can see for part two
fn visibility_map(seats: &Vec<Vec<Option<Seat>>>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut map = HashMap::new();
    let directions: Vec<(i32, i32)> = vec![
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for (row, line) in seats.iter().enumerate() {
        for (col, seat) in line.iter().enumerate() {
            if seat.is_none() {
                continue;
            }

            for (drow, dcol) in &directions {
                let mut i: i32 = row as i32 + drow;
                let mut j: i32 = col as i32 + dcol;
                while i >= 0 && i < seats.len() as i32 && j >= 0 && j < line.len() as i32 {
                    if let Some(_) = seats[i as usize][j as usize] {
                        map.entry((row, col)).or_insert(vec![]).push((i as usize, j as usize));
                        break;
                    }

                    i += drow;
                    j += dcol;
                }
            }
        }
    }

    map
}

fn stable_occupied_count_part2(seats: &Vec<Vec<Option<Seat>>>) -> usize {
    let mut seats = seats.clone();
    let vis_map = visibility_map(&seats);
    let mut result = 0;
    loop {
        seats = sim_step_part2(&seats, &vis_map);
        // println!("{}\n", print_input(&seats));
        let this_occupied = num_occupied(&seats);
        if this_occupied == result {
            return result;
        }
        result = this_occupied;
    }
}

fn sim_step_part2(seats: &Vec<Vec<Option<Seat>>>, vis_map: &HashMap<(usize, usize), Vec<(usize, usize)>>) -> Vec<Vec<Option<Seat>>> {
    let mut out = seats.clone().to_vec();
    for (row, line) in seats.iter().enumerate() {
        for (col, seat) in line.iter().enumerate() {
            if seat.is_none() {
                continue;
            }

            let mut occupied_neighbors = 0;

            for (i, j) in vis_map[&(row, col)].iter() {
                if seats[*i][*j] == Some(Seat::Occupied) {
                    occupied_neighbors += 1;
                }
            }

            // If a seat is empty (L) and there are no occupied seats
            // adjacent to it, the seat becomes occupied.
            if *seat == Some(Seat::Empty) && occupied_neighbors == 0 {
                out[row][col] = Some(Seat::Occupied);
            }

            // If a seat is occupied (#) and four or more seats
            // adjacent to it are also occupied, the seat becomes
            // empty.
            if *seat == Some(Seat::Occupied) && occupied_neighbors >= 5 {
                out[row][col] = Some(Seat::Empty);
            }
        }
    }
    out
}

fn num_occupied(seats: &Vec<Vec<Option<Seat>>>) -> usize {
    seats
        .into_iter()
        .flat_map(|line| line.into_iter().filter(|s| **s == Some(Seat::Occupied)))
        .count()
}

#[derive(Debug, Clone, PartialEq)]
enum Seat {
    Empty,
    Occupied,
}

fn parse_input(input: &str) -> Vec<Vec<Option<Seat>>> {
    input
        .lines()
        .map(|line| line
             .chars()
             .map(|c| {
                 if c == 'L' {
                     Some(Seat::Empty)
                 } else {
                     None
                 }
             })
             .collect()
        ).collect()
}

fn print_input(seats: &Vec<Vec<Option<Seat>>>) -> String {
    seats
        .iter()
        .map(|line| line
             .iter()
             .map(|s| match s {
                 None => ".",
                 Some(Seat::Empty) => "L",
                 Some(Seat::Occupied) => "#",
             })
             .collect::<Vec<&str>>()
             .join("")
        )
        .collect::<Vec<String>>()
        .join("\n")
}

const _EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

const INPUT: &str = "LLLLLLLLLLLL.LLLLLLLLLL.LLLL.LLLL.LLLLLLLLLLL.LLLL.LLLLLLL.LL.LLLLLLLL.LLL.LLLLLLLLLLL.LLLLLLL
LLLLLLLLLLL.LLLL.L..LLL.LL.LLLLLLLLLLL.LLLLLL.LLLLLLLL.L.LLLLL.LLLLLL..LLL.LLLLLLLLLLLLLLLLLLL
LLLLLL.LLLL.LLLL.L.LLLL.LLL.LLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLL..LLLLLLLLLLLLLLLLLLLLLLLLLLL.LLL
LLLLLLLLLLL.LL.LL..LLLL.LLLL.L.LLLLLLL.LLLLLL.LLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLL.L.L.L..LLLLLLLLLL..LLL..LLLL.LLLL.LLLLLLLL.LLLLLL.LLLLLL.LLLLLLLLLLLLLLL.L.LLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLL.LLLL.LLLL.LL.LLLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLL.LLL.LLLL.LLLLLLL.LLLLLL
.L.L....LL......LL..LL.....LLLLL.......L....LL......L.L..L..L..L.L.LL.....LL.......L.L....L..L
LLLLLL.LLLL.LLLLLL.LLLL.LLLLLLLLLLLLLL.LLLLLL.LLLLLLLL.LLLL.L.LLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL
LLLLLL..LLL.LLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLL..LLLLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLLL.LL.LLLL
LLLLLL.LLLL.LLLLLL.L.LL.LLLL.LLLLLLLLL.LLLLLLLLLLLL.LL.LLLLLLLLLL..LLLLLL.LLLLLLLLLLLLLLLLLLLL
LL..LL.LLLL.LLLLLL.LLLLLL.LL.LLLLL.LLLLLLLLLL.LLLLLLLLLLLLLLL.LL.LLLLL.LLLLLLLLLLLLLLLLLLLLLLL
L......LLLLL.L....LL...L..........L..L...L.......LL.L.L.L........L......L..L...L...LL.LL...LL.
LLLLLL.L.LL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLL..LLL.LLL.LLLLLLLLLLLLLLLLLLL.LLL
.LLL.L.LLLL.L.LLLLL.LLL.LLLL.LLLLLLL.LLLLLLLL.LLLLLLLLLL.LLLL.LLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLL
LLLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LL.LLL.LLLL.LLL.LLLLLL..LLL.LLL.LL.LLL.L..L.LLLL.LLLLLL
LLLLLL.LLLL.LLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LL
LLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLL..LLLLL.LL.LLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLL
LLLLLL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLL.LLL.L.LL.LLLLLL..LLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLL..LL.L.LLLLLL.LLLL.LLLL.LLLLLLLLL.LLLLLLLLLLLLLLL.LLL.LL.LLLLLLLL.LLLLLLLLLLLLLLLLLL.LLLL
LLLLLL.LLLL.LLLLLL.LLLL.LLLL.LLLLLLLLL.LLLLLL.LLLLLLLL.LLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLL
.LL.......L.LL.L.L.........L..L...L..LL..L.....LL...LLLL.......L.........L.L..L...L.LL.LL...L.
LLLLLL.LLLLLLLLLL..LLLLLLLLL.LLLLLLLLL.LL.LLLLLLLLL.LL.LLLLLLL.LLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLL.LL.LLL..LLLLLL.LLLLLLLLL.LLLLLLLLL.LL.LLL.LLLLLLLL.LLLLLL.LLLLLLLL.LL.LLLLL.LLLLLLLLLLLLLL
LLLLLL.LLLLLLLLLLL.LLLL.LLLLLLLLLLLLLL..LLL.L.LLLLLLLL.LLLLLLLLLLLLLLL.L...LLLL.LLLLLLLLLL.LLL
LLLLLLLLLLLLLLLLLL.LLLL..L.L.LLLLL.LL..LLLLLL.LLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL
LLLLLLLLLLL.LLLLLLL.LLLLLLLL.LLLLLLLLL.LLL.LL.LLLL.LLL.LLLLLL.LLLLLLL..LLLLLLLLLLLLLLLLLL.LLLL
LLLLLLLLLLL.LLLLLL.LLLLLLLLL.L.LLLLLLLLL.LLLL.LLLLLLL..LLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLLLLL.L.LLLLLLLLLLLLLLLL.LL.LLL.LL.L.LLLL.LLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLLLLL.L.LLLL.LLLLLLLLL.LLLLLL.LLLLLLLL.LLLLLL.LLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLL
LLLLLL.L.LL.LLLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLL..LLLLLLL.LL.LLL.LLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLL
.L........LL..L..LLL.LL.LL..L.....L..L........LL.LL.L..L.L..LL.L......L..L.L..L.L...L..L....L.
LLLLLL.LLL.LLLL.LLLLLLL.LLL.L.LL.LLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL
LLLL.LLLLL.LLLLLLL.LLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLL.LLLLLLL.LLLLLL
LLLLLL..LLLLLLLLLL.LLLLLLLLL.L.LLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLL.LLLLL...LLL.LLLLLLLLLLLLLLLLL
LLLLLLLLLLLLLLLLLL.LLLL.LLLL.LLLLLLLLLL.LLLL..LLLLLLLL.LLLLLLLLLLLLLLL.L.LLLLLL.LLLLLLLLLLLLLL
LLLLLLLLLLL.LLLLLLLLLLL.L.LL.LLLLLLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL
LLLLLLLLLL..LLLLLL.LLLL.LLLLLLLLLLLLLL.LL.LLL.LL.LLLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL
LLL.LL.LLLLLLLL.LL.LLLL.LLLL.L.LLLLLLLLLLLLLL.LLLLLLLL.L.LLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL
....LL.....L..L.L..L.....LLL....LL.LL.L............L..LL.....LL..LL..L....L..L.L.L........LL..
LLLLLL.LLLL.LLLLLL.LLLLL.LLL.LLLLLLLLL.LLLLLL.LLLLLLLL.LLLLLL.LLLLLLLL.L.LLLLLL.LLLLLLLLLLLLLL
LLLLLL.LLLL.LLL.LLLLLLL.LLLLLLLLLLL.LL.LLLLLL.LLL.LLLLLLLLLLL.LLLLLLLL.LLLL.LLLLLLLLLLLLL.LLLL
LLLL.LLLLLLLLLLL.L..LLLLLLLLLLLL.LLLLL.LLLLLL.LLLLLLLL.LLLLLL.LLLL.LL..LLLLLLLL.LLLLLLLLLLL.LL
LLLLLL.LLLLLLLLLLL.LLLL.LLLL.LLLLLLLL..LLLLL..LLLLLLLLLL.LLL.LLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLL.LLLLLLLLLLLLLLLL..L.LL.LLL.LLLLLLL.LLLLLLLL.LLLLLLL.LLL.LLLLLLLLLLLLLLLLLLL
LLLLLL.LLLL.L.LLLL.LLLLLLL.L.LLLL.LLLLLLL.LL..LLLLLLLL.LLLLLL.LLL.LLLL.LL..LLLL.LLLLLLLLLLL.LL
LLLLLL.LLLL.LLLLLL.LLLLLLL.L.L.LLLLL.L.LLLLLLLLLLLLLLLLLLLLLL.LLLLLL.L.LL.LLLLL.LLLL.LLLLLLLLL
.LLL....L.LLLL..L.LL.......LL.L...LL.L...LL....L..L..L..LLLL....L....L..L...L....L...L.LL.....
LLLLLL.LLLL.LLLLLL.LLLLLLLLLLLLL.LLLLL..LLLLL.LLLLLL.LLLLLLLLLL.LLLLLLLLLLLLLLLLLL.LLLLLLLLLLL
LLLLL..LLLLLLLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLL.LLLLLLLLLLLLL.LLL.LLLLLLLLLLLLLL
..LL.LLLL.LL.LLLLL.L.LL.LLLL.LLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLLL..LLLLLLLL.LLLLLLLLL.LL.L
LLLLLL..LLL.LLLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLL.LLLL.LLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL
LLLLLLLLLLL.LLLLLLLLLLL.LLLLLLLLLLLLLL.LL.LLL.LLLLLLLL.LLLLLL.LLLLLLLL.LLLL.L.LLLLLLLLLLLL.LLL
LLLLLLLLLLLL.LL.LL.LLLL.LLLL.L.LL.L.LLLLLLL.L.LLLLLLLL.LLLLLL.LLLL.LLL.LLLLLLLL.LLLLLLLLLLLLLL
......L...L....L..............L.L....L..L.LL.L..LL....LLL........L..LL.L..L.L.LLL.........L.L.
LLLLLL.LLLL.LLLLLL.L.LL.LLLL.LLLL.LLLLLL.LLLL.LLLLLLLL.LLL.LL.LLLLLLLL.LL.LLLLLLLLLLLLLLLLLLLL
LL.LLLLLLLLL.LLLLLLLLLL.LLLL.LLLLLLLLL.LLLLLLLLLLLLLLL.LLLL.L.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLL.L
LLLLLL.LLLL.LLLLLL.LLLL.LLLL.LLLLLLLLL.L.LLLL.LLLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
L.LLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLL.LLLLLL.LLL.LLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLL..LLLL..LLLL
LL.LLL.LLLLLLLLLLL.LLLLLLLLL..LLLLLLLL.LLLLLL.LLLL..LL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LL
LLLLLLLLLLLLLLLLLLLLLL..LLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLL
LLLLLLLLL.L..LLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLL.LLLL.LLLLLLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLL.L.LLLL.LLLL.L.L.LLLLLLL.LLLLLLLLLLLL.LLL.LLLLLLLL..LLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLL
LLL.LL.LLLL.LLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLLLLLL.LLL.LLLLLLLLLLLLLL
.L.......L..L...L..LL......L..L..L..L...L.L........LLL......L......LL.....L.L......L.L.LL....L
LLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLL.LLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLL.LLLLLL
LLLL.LLLLLLLLLLLLLLLL..LLLLL.LLLLLLLLL..LLLLL.LLLLLLLL.LLLLLLLLLLLLL.L.LLLLLLLLLLLLLLLLLLLLLLL
.LLLLL.LLLL.LL.LLL.LLLL.LLLL.LLLLLLLLL.LLLL.L.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLLLLLLLL.LLL.LLLLLLLLL.LLLLLL.LLLLLLLL.LLLL...LLLLLLLL.L.LLLLLLLLLLLLLLLLLLLLL
LLLLLLLLLLL.LLLLLL.LLLLLLLLL.LLLL.LLLL.LL.LLL.LLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLL
......L.LLLL...LLL.L..L......L..L.L.L....L.......LLL....L..LLLL.LLL..LL..L.L.L........LLL.L...
LLLLLL.LLLL.LLLLLL.LLLL.LLLL..LLLLLLLL.LLLLLL.LLLLLLLL..LLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLL.LL.LLLL.LLLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLL.LLLLL.LLLLLLLL
LLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLL.LLLLLLLLLLLLLLLL.LLLL.LLLLLLLLL.LL.LLLLLLLLLLLLLLLL.LL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLL....L.LLLLLL.LLLL.LLL.LLLLLLLLLL.LLLLLL.LLLLLLLL.LLLLLL.LLLLL..L.LLLLLLLLLLLLLLLLLLLLL.L
LLLLLLLL.LL.LLLLLLLLLLLLLL.L.LLLLLLLLL.LLLLLL..LLLLLLL.LLLLLLLL.LLLLLLLLLL.LLLL.LLLLLLLLLLL.LL
LLLLLL.LLLLLLLLLLLLLLLL.LLLL.LLLLLLLLLLLLLL...LLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL
.L..L....L.L..L..LL...L.L.LLLLLL...LLL.LL......LLL...L..LLLL.LLLLL.L...LLL...L.LL..L.L...L.L.L
LLLLLL.LLL..LLLLLL.LLLL.LLLL.LLLLLLLLL.LLLLLLLLLLLLLLL..LLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLLLLLLL.LLLL.LLLLLLLLLLLLLLL..LLLLLLLL.LLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLLLLLLL.LLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLL..LLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLL.LLLLLLLLLLLLLLLLL.L.L.LLLL.LLLLLLLLLLLLLLL.LL.LLLLLLLLLLLLLL..LLLLLLLLLLLLL
L.LLLL.LLLL.LLL.LLLLLLLLLLLL.LLLLLLLLL.LLLLL..LLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL
LLLLL.LLLLL.LLLL.L.LLLL.LLLL.LLLLL.LLL.LLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL
LLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLL.LLLLLL.LLLLLLLL.LLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLL.
L....L.....L..L......L....L.L...L.L.....L.L...LL......L....LL...L....LL...LL.......LLL...L.L.L
LLLLLLLLLLL.LLLLLLLL.LL.LLL..LLLLLLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLLL.LLL.LLLL.LLLLLLLLLLLLLL
LLLLLLLLLLLLLLLLLLLLLLLLLLL..LLLLLLLLL.LLLLLL..LLLLLLL.LLLLLL.L.L.LLLLLLLLL.LLL.LLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLL.LLLL.LL.LLLLLLLLLLL.LLLLLLLL.LLLLLL.LLLLL...LLLLLLLLLLLLLLLL.LLLLLLLLLLLLL.
LLLLLL.LLLL.LLLLLL.LL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLL.LLLLLL.L.LLLLLLLLLLLLLL
LLLLLL.LLLL.LLLLLLLLLLL.LLLL.LLLLL.LLLLLLLLLL.LLLLLLLL..LLLLL.LL.LLLLL.LLL.LL.L.LLLLLLLLLLLLLL";
