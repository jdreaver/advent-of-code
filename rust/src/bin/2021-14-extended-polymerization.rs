use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = parse_input(INPUT);
    println!("part1: {}", compute_solution(&input, 10));
    println!("part2: {}", compute_solution(&input, 40));
}

fn compute_solution(input: &Input, steps: usize) -> usize {
    let counts = polymer_steps_count_pairs(&input.template, &input.rules, steps);
    let mut sorted_counts = counts
        .iter()
        .map(|(_, &count)| count)
        .collect::<Vec<usize>>();
    sorted_counts.sort();

    let max_count = sorted_counts.last().expect("last elem");
    let min_count = sorted_counts[0];
    max_count - min_count
}

fn polymer_steps_count_pairs(start: &[char], rules: &[Rule], num_steps: usize) -> HashMap<char, usize> {
    let pair_results = rules
        .iter()
        .map(|rule| (rule.pair, rule.result))
        .collect::<HashMap<_, _>>();

    let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();
    for (a, b) in start.iter().tuple_windows() {
        *pair_counts.entry((*a, *b)).or_insert(0) += 1;
    }

    for _ in 0..num_steps {
        let mut new_pair_counts: HashMap<(char, char), usize> = HashMap::new();
        for (pair, count) in pair_counts {
            let &replacement = pair_results.get(&pair).expect("couldn't find replacement");
            *new_pair_counts.entry((pair.0, replacement)).or_insert(0) += count;
            *new_pair_counts.entry((replacement, pair.1)).or_insert(0) += count;
        }
        pair_counts = new_pair_counts;
    }

    // Only count second character for each pair, but add 1 for the
    // first character (which never changes).
    let mut char_counts = HashMap::new();
    char_counts.insert(start[0], 1);
    for ((_, b), count) in pair_counts {
        //*char_counts.entry(a).or_insert(0) += count;
        *char_counts.entry(b).or_insert(0) += count;
    }
    char_counts
}

// Old, naive solution that rebuild the vector every time
// fn polymer_steps(start: &[char], rules: &[Rule], num_steps: usize) -> Vec<char> {
//     let pair_results = rules
//         .iter()
//         .map(|rule| (rule.pair, rule.result))
//         .collect::<HashMap<_, _>>();

//     let mut output = start.to_vec();
//     for i in 0..num_steps {
//         println!("i: {}, len: {}", i, output.len());
//         let mut new_output = vec![output[0]];
//         for (a, b) in output.iter().tuple_windows() {
//             let replacement = pair_results.get(&(*a, *b)).expect("couldn't find replacement");
//             new_output.push(*replacement);
//             new_output.push(*b);
//         }
//         output = new_output;
//     }

//     output
// }

// fn char_counts(chars: &[char]) -> HashMap<&char, usize> {
//     let mut counts = HashMap::new();
//     for c in chars.iter() {
//         *counts.entry(c).or_insert(0) += 1;
//     }
//     counts
// }

#[derive(Debug)]
struct Input {
    template: Vec<char>,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    pair: (char, char),
    result: char,
}

fn parse_input(input: &str) -> Input {
    let mut input_lines = input.lines();

    let template = input_lines
        .next()
        .expect("no first line of input")
        .chars()
        .collect::<Vec<char>>();

    assert_eq!(input_lines.next(), Some(""));

    let rules = input_lines
        .map(|line| {
            let (pair_str, result_str) = line.split_once(" -> ").expect("split on arrow");

            let pair_vec = pair_str.chars().collect::<Vec<char>>();
            assert_eq!(pair_vec.len(), 2);
            let pair = (pair_vec[0], pair_vec[1]);

            let result_vec = result_str.chars().collect::<Vec<char>>();
            assert_eq!(result_vec.len(), 1);
            let result = result_vec[0];

            Rule { pair, result }
        })
        .collect();

    Input { template, rules }
}

const _EXAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

const INPUT: &str = "PKHOVVOSCNVHHCVVCBOH

NO -> B
PV -> P
OC -> K
SC -> K
FK -> P
PO -> P
FC -> V
KN -> V
CN -> O
CB -> K
NF -> K
CO -> F
SK -> F
VO -> B
SF -> F
PB -> F
FF -> C
HC -> P
PF -> B
OP -> B
OO -> V
OK -> N
KB -> H
PN -> V
PP -> N
FV -> S
BO -> O
HN -> C
FP -> F
BP -> B
HB -> N
VC -> F
PC -> V
FO -> O
OH -> S
FH -> B
HK -> B
BC -> F
ON -> K
FN -> N
NN -> O
PH -> P
KS -> H
HV -> F
BK -> O
NP -> S
CC -> H
KV -> V
NB -> C
NS -> S
KO -> V
NK -> H
HO -> C
KC -> P
VH -> C
VK -> O
CP -> K
BS -> N
BB -> F
VV -> K
SH -> O
SO -> N
VF -> K
NV -> K
SV -> O
NH -> C
VS -> N
OF -> N
SP -> C
HP -> O
NC -> V
KP -> B
KH -> O
SN -> S
CS -> N
FB -> P
OB -> H
VP -> B
CH -> O
BF -> B
PK -> S
CF -> V
CV -> S
VB -> P
CK -> H
PS -> N
SS -> C
OS -> P
OV -> F
VN -> V
BV -> V
HF -> B
FS -> O
BN -> K
SB -> N
HH -> S
BH -> S
KK -> H
HS -> K
KF -> V";
