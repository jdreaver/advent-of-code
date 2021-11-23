use std::collections::{HashMap, HashSet};

fn main() {
    let active = parse_input(INPUT);

    let mut part1_active = active.clone();
    for _ in 0..6 {
        part1_active = simulate_3d_cycle(&part1_active, false);
    }
    println!("part1: {}", part1_active.len());

    let mut part2_active = active.clone();
    for _ in 0..6 {
        part2_active = simulate_3d_cycle(&part2_active, true);
    }
    println!("part2: {}", part2_active.len());
}

fn simulate_3d_cycle(active_cubes: &HashSet<Cube>, is_4d: bool) -> HashSet<Cube> {
    // First count how many active neighbors each cube has
    let mut active_neighbors: HashMap<Cube, u8> = HashMap::new();
    for active in active_cubes.iter() {
        let neighbors = if is_4d {
            neighbors_4d(active)
        } else {
            neighbors_3d(active)
        };
        for neighbor in neighbors {
            *active_neighbors.entry(neighbor).or_insert(0) += 1;
        }
    }

    // N.B. We could include all of the active cubes with zero
    // neighbors if we needed to, but if an active cube has no
    // neighbors then it becomes inactive, so we can ignore them.

    // Iterate over all of the cubes now and flip
    let mut output: HashSet<Cube> = HashSet::new();
    for (cube, &count) in active_neighbors.iter() {
        // If a cube is active and exactly 2 or 3 of its neighbors are
        // also active, the cube remains active. Otherwise, the cube
        // becomes inactive.
        let keep_active = active_cubes.contains(cube) && (count == 2 || count == 3);

        // If a cube is inactive but exactly 3 of its neighbors are
        // active, the cube becomes active. Otherwise, the cube
        // remains inactive.
        let make_active = !active_cubes.contains(cube) && count == 3;

        if keep_active || make_active {
            output.insert(cube.clone());
        }
    }

    output
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

// impl Add for Cube {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z,
//         }
//     }
// }

fn neighbors_3d(cube: &Cube) -> Vec<Cube> {
    let mut neighs = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if !(dx == 0 && dy == 0 && dz == 0) {
                    neighs.push(Cube {
                        x: cube.x + dx,
                        y: cube.y + dy,
                        z: cube.z + dz,
                        w: cube.w,
                    });
                }
            }
        }
    }
    assert_eq!(neighs.len(), 26);
    neighs
}

fn neighbors_4d(cube: &Cube) -> Vec<Cube> {
    let mut neighs = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if !(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                        neighs.push(Cube {
                            x: cube.x + dx,
                            y: cube.y + dy,
                            z: cube.z + dz,
                            w: cube.w + dw,
                        });
                    }
                }
            }
        }
    }
    assert_eq!(neighs.len(), 80);
    neighs
}

fn parse_input(input: &str) -> HashSet<Cube> {
    let mut active = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert(Cube {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                    w: 0,
                });
            }
        }
    }
    active
}

const _EXAMPLE: &str = ".#.
..#
###";

const INPUT: &str = "##.#....
...#...#
.#.#.##.
..#.#...
.###....
.##.#...
#.##..##
#.####..";
