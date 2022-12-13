use std::iter::Peekable;

fn main() {
    let input = parse_input(_EXAMPLE);
    println!("part 1: {}", part1(&input));
}

fn part1(pairs: &[(PacketValue, PacketValue)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(i, (first, second))| {
            println!("comparing {}: {:?}, {:?}", i+1, first, second);
            let result = packets_in_order(first, second);
            println!("result: {}", result);
            result
        })
        .map(|(i, _)| i + 1)
        .sum()
}

fn packets_in_order(first: &PacketValue, second: &PacketValue) -> bool {
    // TODO: We need three-valued logic here: true, false, or no decision (continue)
    println!("packets_in_order: {:?}, {:?}", first, second);
    match (first, second) {
        (PacketValue::Num(x), PacketValue::Num(y)) => x <= y,
        (PacketValue::List(xs), PacketValue::List(ys)) => {
            if ys.len() > xs.len() {
                return false;
            }
            xs.iter().zip(ys).all(|(x, y)| packets_in_order(x, y))
        }
        (xs @ PacketValue::List(_), y @ PacketValue::Num(_)) => {
            packets_in_order(xs, &PacketValue::List(vec![y.clone()]))
        }
        (x @ PacketValue::Num(_), ys @ PacketValue::List(_)) => {
            packets_in_order(&PacketValue::List(vec![x.clone()]), ys)
        }
    }
}

#[derive(Debug, Clone)]
enum PacketValue {
    List(Vec<PacketValue>),
    Num(u32),
}

fn parse_input(input: &str) -> Vec<(PacketValue, PacketValue)> {
    let mut lines = input.lines();
    let mut pairs = vec![];
    loop {
        let lhs = parse_packet(lines.next().expect("no line for lhs"));
        let rhs = parse_packet(lines.next().expect("no line for rhs"));
        pairs.push((lhs, rhs));

        match lines.next() {
            None => break,
            Some("") => continue,
            Some(l) => panic!("expected empty line or end, got: \"{}\"", l),
        }
    }
    pairs
}

fn parse_packet(input: &str) -> PacketValue {
    let mut chars = input.chars().peekable();
    assert_eq!(
        chars.next(),
        Some('['),
        "expected top level packet to be list"
    );

    let val = PacketValue::List(parse_packet_list(&mut chars));

    assert_eq!(chars.next(), None);

    val
}

fn parse_packet_list<I>(chars: &mut Peekable<I>) -> Vec<PacketValue>
where
    I: Iterator<Item = char>,
{
    let mut elems: Vec<PacketValue> = vec![];

    while let Some(c) = chars.next() {
        match c {
            '[' => elems.push(PacketValue::List(parse_packet_list(chars))),
            ',' => continue,
            ']' => return elems,
            x => elems.push(PacketValue::Num(
                x.to_digit(10)
                    .unwrap_or_else(|| panic!("unable to parse num {}", x)),
            )),
        }
    }

    panic!("malformed list")
}

const _EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
