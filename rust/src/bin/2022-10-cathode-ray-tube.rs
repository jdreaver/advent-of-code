fn main() {
    let instructions = parse_input(INPUT);
    let cycles = simulate_cycles(&instructions);
    println!("part 1: {}", part1_signal_strength(&cycles));
}

fn part1_signal_strength(cycles: &[i32]) -> i32 {
    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|cycle| (*cycle as i32) * cycles[*cycle - 1])
        .sum()
}

fn simulate_cycles(instructions: &[Instruction]) -> Vec<i32> {
    let mut x = 1;
    let mut cycles = vec![];
    for instruction in instructions {
        match instruction {
            Instruction::Noop => cycles.push(x),
            Instruction::Addx(dx) => {
                cycles.push(x);
                cycles.push(x);
                x += dx;
            }
        }
    }

    // Add cycle just in case we ended on an addx
    cycles.push(x);

    cycles
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match line.split(' ').collect::<Vec<&str>>()[..] {
            ["noop"] => Instruction::Noop,
            ["addx", num] => Instruction::Addx(num.parse().expect("no parse num")),
            _ => panic!("unknown instruction {}", line),
        })
        .collect()
}

const _EXAMPLE1: &str = "noop
addx 3
addx -5";

const _EXAMPLE2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

const INPUT: &str = "addx 1
addx 5
noop
addx -1
noop
addx 3
addx 29
addx -1
addx -21
addx 5
noop
addx -20
addx 21
addx 2
addx 8
addx -1
noop
noop
noop
noop
addx 6
addx -1
addx -37
addx 40
addx -10
addx -25
addx 5
addx 2
addx 5
noop
noop
noop
addx 21
addx -20
addx 2
noop
addx 3
addx 2
addx -5
addx 12
addx 3
noop
addx 2
addx 3
addx -2
addx -37
addx 1
addx 5
addx 3
addx -2
addx 2
addx 29
addx -22
addx 13
noop
addx -8
addx -6
addx 7
addx 2
noop
addx 7
addx -2
addx 5
addx 2
addx -26
addx -11
noop
noop
addx 6
addx 1
addx 1
noop
addx 4
addx 5
noop
noop
addx -2
addx 3
noop
addx 2
addx 5
addx 2
addx -22
addx 27
addx -1
addx 1
addx 5
addx 2
noop
addx -39
addx 22
noop
addx -15
addx 3
addx -2
addx 2
addx -2
addx 9
addx 3
noop
addx 2
addx 3
addx -2
addx 2
noop
noop
noop
addx 5
addx -17
addx 24
addx -7
addx 8
addx -36
addx 2
addx 3
addx 33
addx -32
addx 4
addx 1
noop
addx 5
noop
noop
addx 20
addx -15
addx 4
noop
addx 1
noop
addx 4
addx 6
addx -30
addx 30
noop
noop
noop
noop
noop";
