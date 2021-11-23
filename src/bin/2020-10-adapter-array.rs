use itertools::Itertools;
use std::cmp;

fn main() {
    let input = parse_input(_EXAMPLE1);
    println!("part1: {}", part1_solution(&input));

    // TODO: Part 2 is wrong
    println!("part2: {}", valid_arrangements(&input));
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|c| c.parse::<u32>().expect("parsing input num"))
        .collect()
}

fn part1_solution(input: &[u32]) -> u32 {
    // Add 0 to input (wall) and then sort
    let mut input = input.clone().to_vec();
    input.push(0);
    input.push(input.iter().max().expect("max") + 3);
    input.sort();

    let mut diffs_1 = 0;
    let mut diffs_3 = 0;

    for (x, y) in input.iter().tuple_windows() {
        if y - x == 1 {
            diffs_1 += 1;
        } else if y - x == 3 {
            diffs_3 += 1;
        }
    }
    println!("1s: {}, 3s: {}", diffs_1, diffs_3);
    diffs_1 * diffs_3
}

fn valid_arrangements(input: &[u32]) -> u64 {
    let mut input = input.clone().to_vec();
    input.push(0);
    input.push(input.iter().max().expect("max") + 3);
    input.sort();

    let mut cache = vec![None; input.len()];

    paths_to_end(0, &input, &mut cache)

    // let mut arrangements: u64 = 1;
    // for i in 0..input.len() {
    //     // If we can skip any of the next elements, multiply
    //     // arrangements by 2.
    //     let mut j = i + 2;
    //     while j < input.len() {
    //         //println!("iter [{}] = {}, [{}] = {}", i, input[i], j, input[j]);
    //         if input[j] - input[i] < 3 {
    //             arrangements *= 2;
    //             j += 1;
    //         } else {
    //             break;
    //         }
    //     }
    // }

    // arrangements
}

fn paths_to_end(i: usize, input: &[u32], cache: &mut Vec<Option<u64>>) -> u64 {
    if i == input.len() - 1 {
        return 1;
    }
    if let Some(x) = cache[i] {
        return x;
    }

    let mut paths = paths_to_end(i + 1, input, cache);
    for j in (i + 1)..cmp::min(i + 3, input.len()) {
        if input[j] - input[i] < 3 {
            paths += paths_to_end(j + 1, input, cache);
        }
    }
    paths
}

const _EXAMPLE1: &str = "16
10
15
5
1
11
7
19
6
12
4";

const _EXAMPLE2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

const INPUT: &str = "99
3
1
11
48
113
131
43
82
19
4
153
105
52
56
109
27
119
147
31
34
13
129
17
61
10
29
24
12
104
152
103
80
116
79
73
21
133
44
18
74
112
136
30
146
100
39
130
91
124
70
115
81
28
151
2
122
87
143
62
7
126
95
75
20
123
63
125
53
45
141
14
67
69
60
114
57
142
150
42
78
132
66
88
140
139
106
38
85
37
51
94
98
86
68";
