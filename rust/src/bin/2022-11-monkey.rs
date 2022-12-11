fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1_monkey_business(&input));
    println!("part 2: {}", part2_monkey_business(&input));
}

fn part1_monkey_business(monkeys: &[Monkey]) -> usize {
    monkey_business(monkeys, 20, None)
}

fn part2_monkey_business(monkeys: &[Monkey]) -> usize {
    let modulo = monkeys.iter().map(|monkey| monkey.test_divisible).product();
    monkey_business(monkeys, 10_000, Some(modulo))
}

fn monkey_business(monkeys: &[Monkey], rounds: usize, modulo: Option<u64>) -> usize {
    let mut inspected: Vec<usize> = simulate_rounds(monkeys, modulo, rounds)
        .iter()
        .map(|items| items.num_items_inspected)
        .collect();
    inspected.sort();

    let n = monkeys.len();
    inspected[n - 1] * inspected[n - 2]
}

fn simulate_rounds(monkeys: &[Monkey], modulo: Option<u64>, rounds: usize) -> Vec<MonkeyItems> {
    let mut items = monkeys
        .iter()
        .map(|monkey| MonkeyItems {
            num_items_inspected: 0,
            objects: monkey.starting_items.clone(),
        })
        .collect::<Vec<MonkeyItems>>();
    (0..rounds).for_each(|_| {
        items = simulate_round(monkeys, &items, modulo);
    });
    items
}

#[derive(Debug, Clone)]
struct MonkeyItems {
    num_items_inspected: usize,
    objects: Vec<u64>,
}

fn simulate_round(
    monkeys: &[Monkey],
    items: &[MonkeyItems],
    modulo: Option<u64>,
) -> Vec<MonkeyItems> {
    let mut items = items.to_vec();
    monkeys.iter().enumerate().for_each(|(i, monkey)| {
        let mut num_items_inspected = 0;
        items[i].objects.clone().iter().for_each(|object| {
            num_items_inspected += 1;
            let mut item = apply_operation(*object, &monkey.operation);
            match modulo {
                None => item /= 3,
                Some(m) => item %= m,
            }

            let target = if item % monkey.test_divisible == 0 {
                monkey.true_target
            } else {
                monkey.false_target
            };
            items[target].objects.push(item);
        });
        items[i].num_items_inspected += num_items_inspected;
        items[i].objects = Vec::new();
    });

    items
}

fn apply_operation(item: u64, op: &Operation) -> u64 {
    let lhs = item;
    let rhs = match op.rhs {
        OperationRHS::Old => item,
        OperationRHS::Num(x) => x,
    };
    match op.operator {
        Operator::Add => lhs + rhs,
        Operator::Multiply => lhs * rhs,
    }
}

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<u64>,
    operation: Operation,
    test_divisible: u64,
    true_target: usize,
    false_target: usize,
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    rhs: OperationRHS,
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
enum OperationRHS {
    Old,
    Num(u64),
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut lines = input.lines();
    let mut monkeys = Vec::new();

    loop {
        assert!(lines.next().expect("no Monkey line").starts_with("Monkey "));

        let (starting_items_key, starting_items_str) = lines
            .next()
            .expect("no starting items")
            .split_once(": ")
            .expect("no :");
        assert_eq!(starting_items_key, "  Starting items");
        let starting_items: Vec<u64> = starting_items_str
            .split(", ")
            .map(|s| s.parse::<u64>().expect("no parse item"))
            .collect();

        let (operation_key, operation_str) = lines
            .next()
            .expect("no operation")
            .split_once(" = ")
            .expect("no =");
        assert_eq!(operation_key, "  Operation: new");

        let operation =
            if let ["old", op_str, rhs_str] = operation_str.split(' ').collect::<Vec<&str>>()[..] {
                let operator = match op_str {
                    "+" => Operator::Add,
                    "*" => Operator::Multiply,
                    _ => panic!("unknown operator {}", op_str),
                };

                let rhs = if rhs_str == "old" {
                    OperationRHS::Old
                } else {
                    OperationRHS::Num(rhs_str.parse().expect("no parse RHS num"))
                };

                Operation { operator, rhs }
            } else {
                panic!("no parse operation {}", operation_str)
            };

        let (test_key, test_str) = lines
            .next()
            .expect("no test")
            .split_once(": ")
            .expect("no :");
        assert_eq!(test_key, "  Test");

        let test_divisible = match test_str.split(' ').collect::<Vec<&str>>()[..] {
            ["divisible", "by", num] => num.parse().expect("no parse test"),
            _ => panic!("no parse test str: \"{}\"", test_str),
        };

        let (true_key, true_str) = lines
            .next()
            .expect("no true")
            .split_once(": ")
            .expect("no :");
        assert_eq!(true_key, "    If true");

        let true_target = match true_str.split(' ').collect::<Vec<&str>>()[..] {
            ["throw", "to", "monkey", num] => num.parse().expect("no parse test true"),
            _ => panic!("no parse test str: \"{}\"", test_str),
        };

        let (false_key, false_str) = lines
            .next()
            .expect("no false")
            .split_once(": ")
            .expect("no :");
        assert_eq!(false_key, "    If false");

        let false_target = match false_str.split(' ').collect::<Vec<&str>>()[..] {
            ["throw", "to", "monkey", num] => num.parse().expect("no parse test false"),
            _ => panic!("no parse test str: \"{}\"", test_str),
        };

        monkeys.push(Monkey {
            starting_items,
            operation,
            test_divisible,
            true_target,
            false_target,
        });

        match lines.next() {
            None => break,
            Some(line) => assert_eq!(line, ""),
        }
    }

    monkeys
}

const _EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

const INPUT: &str = "Monkey 0:
  Starting items: 93, 54, 69, 66, 71
  Operation: new = old * 3
  Test: divisible by 7
    If true: throw to monkey 7
    If false: throw to monkey 1

Monkey 1:
  Starting items: 89, 51, 80, 66
  Operation: new = old * 17
  Test: divisible by 19
    If true: throw to monkey 5
    If false: throw to monkey 7

Monkey 2:
  Starting items: 90, 92, 63, 91, 96, 63, 64
  Operation: new = old + 1
  Test: divisible by 13
    If true: throw to monkey 4
    If false: throw to monkey 3

Monkey 3:
  Starting items: 65, 77
  Operation: new = old + 2
  Test: divisible by 3
    If true: throw to monkey 4
    If false: throw to monkey 6

Monkey 4:
  Starting items: 76, 68, 94
  Operation: new = old * old
  Test: divisible by 2
    If true: throw to monkey 0
    If false: throw to monkey 6

Monkey 5:
  Starting items: 86, 65, 66, 97, 73, 83
  Operation: new = old + 8
  Test: divisible by 11
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 6:
  Starting items: 78
  Operation: new = old + 6
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1

Monkey 7:
  Starting items: 89, 57, 59, 61, 87, 55, 55, 88
  Operation: new = old + 7
  Test: divisible by 5
    If true: throw to monkey 2
    If false: throw to monkey 5";
