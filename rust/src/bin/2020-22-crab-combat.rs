use std::collections::VecDeque;

fn main() {
    let (player1, player2) = parse_input(INPUT);
    println!("part1: {}", simulate_until_win(&player1, &player2));
}

type Deck = Vec<u64>;

fn simulate_until_win(player1: &[u64], player2: &[u64]) -> u64 {
    let mut player1_deck = VecDeque::from(player1.to_vec());
    let mut player2_deck = VecDeque::from(player2.to_vec());

    loop {
        let player1_card = player1_deck.pop_front().expect("player1 pop");
        let player2_card = player2_deck.pop_front().expect("player2 pop");

        if player1_card > player2_card {
            player1_deck.push_back(player1_card);
            player1_deck.push_back(player2_card);
        } else {
            player2_deck.push_back(player2_card);
            player2_deck.push_back(player1_card);
        }

        if player1_deck.len() == 0 {
            return deck_score(player2_deck.make_contiguous());
        }
        if player2_deck.len() == 0 {
            return deck_score(player1_deck.make_contiguous());
        }
    }
}

fn deck_score(deck: &[u64]) -> u64 {
    deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| card * (i as u64 + 1))
        .sum()
}

fn parse_input(input: &str) -> (Deck, Deck) {
    let mut input_lines = input.lines();

    assert_eq!(input_lines.by_ref().next(), Some("Player 1:"));

    let mut player1_deck = Vec::new();
    loop {
        let line = input_lines.next().expect("next player 1 line");
        if line.is_empty() {
            break;
        }
        player1_deck.push(line.parse().expect("parse player 1 num"));
    }

    assert_eq!(input_lines.by_ref().next(), Some("Player 2:"));

    let player2_deck = input_lines
        .map(|line| line.parse().expect("parse player 2 num"))
        .collect();

    (player1_deck, player2_deck)
}

const _EXAMPLE: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

const INPUT: &str = "Player 1:
18
19
16
11
47
38
6
27
9
22
15
42
3
4
21
41
14
8
23
30
40
13
35
46
50

Player 2:
39
1
29
20
45
43
12
2
37
33
49
32
10
26
36
17
34
44
25
28
24
5
48
31
7";
