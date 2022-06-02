use itertools::Itertools;
use std::cmp;

fn main() {
    let input = parse_input(INPUT);

    let part1 = count_flashes_steps(&input, 100);
    println!("part1: {}", part1);

    let part2 = first_simultaneous_flash(&input);
    println!("part2: {}", part2);

}

fn count_flashes_steps(input: &[Vec<u32>], steps: usize) -> usize {
    let mut flashes = 0;
    let mut cells = input.to_vec();
    for _ in 0..steps {
        let (new_cells, num_flashes) = simulate_step(&cells);
        cells = new_cells;
        flashes += num_flashes;
    }
    flashes
}

fn first_simultaneous_flash(input: &[Vec<u32>]) -> usize {
    let mut cells = input.to_vec();
    let num_cells = cells.iter().flatten().count();
    let mut step = 1;
    loop {
        let (new_cells, num_flashes) = simulate_step(&cells);
        cells = new_cells;
        if num_flashes == num_cells {
            return step;
        }
        step += 1;
    }
}

fn simulate_step(input: &[Vec<u32>]) -> (Vec<Vec<u32>>, usize) {
    let mut output = input.to_vec();

    // Increase energy level by 1 everywhere
    let nrows = output.len();
    let ncols = output[0].len();
    let mut flash_queue: Vec<(usize, usize)> = Vec::new();
    for i in 0..nrows {
        for j in 0..ncols {
            output[i][j] += 1;
            if output[i][j] > 9 {
                flash_queue.push((i, j));
            }
        }
    }

    // Propagate flashes
    let mut flashed: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];
    while let Some((i, j)) = flash_queue.pop() {
        if flashed[i][j] {
            continue;
        }
        flashed[i][j] = true;

        for (i, j) in neighbors(i, j, nrows, ncols) {
            output[i][j] += 1;
            if output[i][j] > 9 {
                flash_queue.push((i, j));
            }
        }
    }

    // Apply flashes
    for i in 0..nrows {
        for j in 0..ncols {
            if output[i][j] > 9 {
                output[i][j] = 0
            }
        }
    }

    let num_flashes = flashed
        .iter()
        .flatten()
        .filter(|&&c| c)
        .count();
    (output, num_flashes)
}

fn neighbors(i: usize, j: usize, nrows: usize, ncols: usize) -> impl Iterator<Item = (usize, usize)> {
    let i_lower = i.saturating_sub(1);
    let j_lower = j.saturating_sub(1);
    let i_upper = cmp::min(i + 1, nrows - 1);
    let j_upper = cmp::min(j + 1, ncols - 1);
    (i_lower..=i_upper)
        .cartesian_product(j_lower..=j_upper)
        .filter(move |&(x, y)| !(x == i && y == j))
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line
             .chars()
             .map(|c| c.to_digit(10).expect("parsing input num"))
             .collect()
        )
        .collect()
}

fn _print_cells(cells: &[Vec<u32>]) {
    println!(
        "{}",
        cells
            .iter()
            .map(|row| row
                 .iter()
                 .map(|&c| format!("{:2}", c))
                 .join(" ")
            )
            .join("\n")
    )
}

const _EXAMPLE1: &str = "11111
19991
19191
19991
11111";

const _EXAMPLE2: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

const INPUT: &str = "8826876714
3127787238
8182852861
4655371483
3864551365
1878253581
8317422437
1517254266
2621124761
3473331514";
