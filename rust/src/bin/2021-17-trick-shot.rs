fn main() {
    let target = INPUT;

    let part1 = (target.ymin) * (target.ymin + 1) / 2;
    println!("part1: {}", part1);

    let vx_min = 1;
    let vx_max = 500;
    let vy_min = -300;
    let vy_max = 1000;
    let part2 = count_reachable_velocities(&target, vx_min, vx_max, vy_min, vy_max);
    println!("part1: {}", part2);
}

fn reaches_target(vx: u32, vy: i32, target: &Target) -> bool {
    let mut vx = vx;
    let mut vy = vy;
    let mut x: u32 = 0;
    let mut y: i32 = 0;

    loop {
        x += vx;
        y += vy;

        if y < target.ymin || x > target.xmax {
            return false;
        }
        if target.contains(x, y) {
            return true;
        }

        vx = vx.saturating_sub(1);
        vy -= 1;
    }
}

fn count_reachable_velocities(target: &Target, vx_min: u32, vx_max: u32, vy_min: i32, vy_max: i32) -> usize {
    let mut reachable = 0;
    for vx in vx_min..=vx_max {
        for vy in vy_min..=vy_max {
            if reaches_target(vx, vy, target) {
                reachable += 1;
            }
        }
    }
    reachable
}

#[test]
fn test_reaches_target() {
    assert!(reaches_target(7, 2, &_EXAMPLE));
    assert!(reaches_target(6, 3, &_EXAMPLE));
    assert!(reaches_target(9, 0, &_EXAMPLE));
    assert!(!reaches_target(17, -4, &_EXAMPLE));
}

struct Target {
    xmin: u32,
    xmax: u32,
    ymin: i32,
    ymax: i32,
}

impl Target {
    fn contains(&self, x: u32, y: i32) -> bool {
        x >= self.xmin && x <= self.xmax && y >= self.ymin && y <= self.ymax
    }
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
