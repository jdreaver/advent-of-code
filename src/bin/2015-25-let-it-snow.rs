// https://adventofcode.com/2015/day/25

fn main() {
    let solution = code_iters(20151125)
        // We want to stop _after_ (3010, 3019), which will be (3009, 3020)
        .take_while(|code| !(code.row == 3009 && code.col == 3020))
        .last()
        .unwrap();
    println!("{}", solution.code);
}

#[derive(Debug)]
struct CodeCoordinate {
    row: usize,
    col: usize,
    code: u64,
}

impl CodeCoordinate {
    fn new(code: u64) -> Self {
        CodeCoordinate {
            row: 1,
            col: 1,
            code,
        }
    }

    fn next(&self) -> Self {
        let code = next_code(self.code);
        if self.row == 1 {
            CodeCoordinate {
                row: self.col + 1,
                col: 1,
                code,
            }
        } else {
            CodeCoordinate {
                row: self.row - 1,
                col: self.col + 1,
                code,
            }
        }
    }
}

fn next_code(code: u64) -> u64 {
    (code * 252533).rem_euclid(33554393)
}

fn code_iters(initial_code: u64) -> impl Iterator<Item = CodeCoordinate> {
    std::iter::successors(Some(CodeCoordinate::new(initial_code)), |c| Some(c.next()))
}
