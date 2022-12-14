use itertools::Itertools;

fn main() {
    let input = parse_input(_EXAMPLE);
    println!("{:?}", input);
}

fn simulate_one_sand(cave: &mut Vec<Vec<bool>>) -> bool {
    let mut sand = (500, 0);
    loop {
        if sand.0 == cave[0].len() - 1 || sand.1 == cave.len() - 1 || sand.1 == 0 {
            // Fell off the map
            return false
        }

        if !cave[sand.0 + 1][sand.1] {
            // Fall down
            sand.0 += 1;
        } else if cave[sand.0 + 1][sand.1 - 1] {
            // Fall down left
            sand.0 += 1;
            sand.1 -= 1;
        } else if cave[sand.0 + 1][sand.1 + 1] {
            // Fall down right
            sand.0 += 1;
            sand.1 += 1;
        } else {
            // We are stuck, stay here
            cave[sand.0][sand.1] = true;
            return true
        }
    }
}

fn fill_in_rocks(input: &[Vec<Point>]) -> Vec<Vec<bool>> {
    // Find necessary dimensions
    let all_rocks = input.iter().flatten().collect::<Vec<&Point>>();
    let &max_x = all_rocks
        .iter()
        .map(|(x, _)| x)
        .max()
        .expect("no max x");
    let &max_y = all_rocks
        .iter()
        .map(|(_, y)| y)
        .max()
        .expect("no max y");

    let cave_row: Vec<bool> = std::iter::repeat(false).take(max_x).collect();
    let mut cave: Vec<Vec<bool>> = std::iter::repeat(cave_row).take(max_y).collect();

    for chain in input {
        for (&(x1, y1), &(x2, y2)) in chain.iter().tuples() {
            if x1 == x2 {
                for y in y1..=y2 {
                    cave[x1][y] = true;
                }
            } else if y1 == y2 {
                for x in x1..=x2 {
                    cave[x][y1] = true;
                }
            } else {
                panic!("invalid pair {:?} {:?}", (x1, y1), (x2, y2));
            }
        }
    }

    cave
}

type Point = (usize, usize);

fn parse_input(input: &str) -> Vec<Vec<Point>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pt_str| {
                    let (x_str, y_str) = pt_str.split_once(',').expect("no parse comma");
                    let x = x_str.parse().expect("no parse x");
                    let y = y_str.parse().expect("no parse y");
                    (x, y)
                })
                .collect()
        })
        .collect()
}

const _EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
