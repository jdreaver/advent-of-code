fn main() {
    let target = INPUT;

    let part1 = (target.ymin) * (target.ymin + 1) / 2;
    println!("part1: {}", part1);
}

struct Target {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

const _EXAMPLE: Target = Target {
    xmin: 20,
    xmax: 30,
    ymin: -10,
    ymax: -5,
};

const INPUT: Target = Target {
    xmin: 153,
    xmax: 199,
    ymin: -114,
    ymax: -75,
};
