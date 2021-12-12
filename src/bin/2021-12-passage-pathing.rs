use std::collections::HashMap;

fn main() {
    let cave = parse_input(INPUT);

    let part1 = count_paths(&cave);
    println!("part1: {}", part1);
}

fn count_paths(cave: &Cave) -> usize {
    count_paths_inner(cave, &Node::Start, &mut HashMap::new())
}

fn count_paths_inner(cave: &Cave, node: &Node, seen_count: &HashMap<Node, usize>) -> usize {
    let next = cave.get(node).expect("couldn't find node in cave!");
    let mut count = 0;

    for child in next {
        match child {
            Node::Start => {},
            Node::End => {
                count += 1;
            },
            Node::Small(_) => {
                if seen_count.get(child).is_none() {
                    let mut child_seen = seen_count.clone();
                    child_seen.insert(child.clone(), 1);
                    count += count_paths_inner(cave, child, &child_seen);
                }
            }
            Node::Large(_) => {
                let mut child_seen = seen_count.clone();
                child_seen.insert(child.clone(), 1);
                count += count_paths_inner(cave, child, &child_seen);
            }
        }
    }
    count
}

type Cave<'a> = HashMap<Node<'a>, Vec<Node<'a>>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Node<'a> {
    Start,
    Small(&'a str),
    Large(&'a str),
    End,
}

fn parse_input(input: &str) -> Cave {
    let mut cave: Cave = HashMap::new();
    for line in input.lines() {
        let (node_a_str, node_b_str) = line.split_once("-").expect("no - in line");
        let node_a = parse_node(node_a_str);
        let node_b = parse_node(node_b_str);
        cave.entry(node_a.clone()).or_insert(Vec::new()).push(node_b.clone());
        cave.entry(node_b).or_insert(Vec::new()).push(node_a);
    }
    cave
}

fn parse_node(node_str: &str) -> Node {
    match node_str {
        "start" => Node::Start,
        "end" => Node::End,
        _ if node_str > "Z" => Node::Small(node_str),
        _ => Node::Large(node_str),
    }
}

const _EXAMPLE1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

const _EXAMPLE2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

const _EXAMPLE3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

const INPUT: &str = "pg-CH
pg-yd
yd-start
fe-hv
bi-CH
CH-yd
end-bi
fe-RY
ng-CH
fe-CH
ng-pg
hv-FL
FL-fe
hv-pg
bi-hv
CH-end
hv-ng
yd-ng
pg-fe
start-ng
end-FL
fe-bi
FL-ks
pg-start";
