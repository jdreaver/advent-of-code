fn main() {}


// fn reduce(pair: &Pair) -> Pair {

// }

/// Simpler representation for the problem. Array of (depth, num).
// type PairArray = Vec<(u8, u32)>;

// fn pair_to_array(pair: &Pair) -> PairArray {
//     let mut array = Vec::new();

//     array
// }

// fn array_to_pair(array: &PairArray) -> Pair {
//     todo!()
// }

#[derive(Debug, PartialEq, Clone)]
struct Pair {
    lhs: PairElement,
    rhs: PairElement,
}

impl Pair {
    fn reduce(&self) -> Pair {
        let mut pair = self.clone();

        loop {
            if self.explode_leftmost() {
                continue;
            }

            // If any regular number is 10 or greater, the leftmost
            // such regular number splits. To split a regular number,
            // replace it with a pair; the left element of the pair
            // should be the regular number divided by two and rounded
            // down, while the right element of the pair should be the
            // regular number divided by two and rounded up. For
            // example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes
            // [6,6], and so on.


            // No rule applied, return the new pair
            return pair
        }
    }

    /// If any pair is nested inside four pairs, the leftmost such
    /// pair explodes. To explode a pair, the pair's left value is
    /// added to the first regular number to the left of the exploding
    /// pair (if any), and the pair's right value is added to the
    /// first regular number to the right of the exploding pair (if
    /// any). Exploding pairs will always consist of two regular
    /// numbers. Then, the entire exploding pair is replaced with the
    /// regular number 0.
    fn explode_leftmost(&self) -> (Pair, bool) {
        let (new_pair, lval, rval, changed) = self.explode_leftmost_inner(1);
        assert!(lval.is_none());
        assert!(rval.is_none());
        (new_pair, changed)
    }

    fn explode_leftmost_inner(&self, level: usize) -> (Pair, Option<u32>, Option<u32>, bool) {
        match (self.lhs, self.rhs) {
            (PairElement::Num(x), PairElement::Num(y)) => {
                if level > 4 {
                    return
                }
            },
            _ => {},
        }
        //self.lhs.explode_leftmost_inner(level) || self.rhs.explode_leftmost_inner(level)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum PairElement {
    Num(u32),
    Pair(Box<Pair>),
}

fn parse_pair(line: &str) -> Pair {
    let chars = line.chars().collect::<Vec<char>>();
    let mut i = 0;
    let pair = parse_pair_inner(&chars, &mut i);
    assert_eq!(i, chars.len());
    pair
}

fn parse_pair_inner(chars: &[char], i: &mut usize) -> Pair {
    assert_eq!(chars[*i], '[');
    *i += 1;

    let lhs = parse_pair_element(chars, i);

    assert_eq!(chars[*i], ',');
    *i += 1;

    let rhs = parse_pair_element(chars, i);

    assert_eq!(chars[*i], ']');
    *i += 1;

    Pair { lhs, rhs }
}

fn parse_pair_element(chars: &[char], i: &mut usize) -> PairElement {
    match chars[*i] {
        '[' => PairElement::Pair(Box::new(parse_pair_inner(chars, i))),
        c => {
            // Assume num. Also, all input numbers are a single character
            *i += 1;
            PairElement::Num(c.to_digit(10).expect("pair element digit"))
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(
        parse_pair("[1,2]"),
        Pair {
            lhs: PairElement::Num(1),
            rhs: PairElement::Num(2),
        },
    );

    assert_eq!(
        parse_pair("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]"),
        Pair {
            lhs: PairElement::Pair(Box::new(Pair {
                lhs: PairElement::Pair(Box::new(Pair {
                    lhs: PairElement::Pair(Box::new(Pair {
                        lhs: PairElement::Num(1),
                        rhs: PairElement::Num(2)
                    })),
                    rhs: PairElement::Pair(Box::new(Pair {
                        lhs: PairElement::Num(3),
                        rhs: PairElement::Num(4)
                    }))
                })),
                rhs: PairElement::Pair(Box::new(Pair {
                    lhs: PairElement::Pair(Box::new(Pair {
                        lhs: PairElement::Num(5),
                        rhs: PairElement::Num(6)
                    })),
                    rhs: PairElement::Pair(Box::new(Pair {
                        lhs: PairElement::Num(7),
                        rhs: PairElement::Num(8)
                    }))
                }))
            })),
            rhs: PairElement::Num(9),
        },
    );
}

const INPUT: &str = "[[[[9,5],[9,4]],[[6,5],[7,0]]],4]
[[[5,2],[[7,2],1]],[[[7,5],[0,8]],[[6,9],[7,3]]]]
[[[9,7],[0,1]],9]
[1,[[7,3],[[3,7],[3,2]]]]
[[9,[[0,8],7]],[[3,1],[[6,6],[9,0]]]]
[4,[[4,4],[[7,7],1]]]
[[[[6,2],[5,1]],[[3,3],9]],[7,[[5,7],[5,0]]]]
[[[[4,8],[4,9]],[1,[9,3]]],[1,[1,[6,1]]]]
[[[[4,7],[3,4]],[8,3]],[[3,7],[0,[1,8]]]]
[[[6,[4,8]],[4,5]],[4,[1,3]]]
[[[0,7],0],[[6,[1,8]],[9,[7,9]]]]
[[[[4,8],[3,9]],[4,5]],[1,1]]
[[[4,2],[0,[6,7]]],[[[1,8],2],[8,8]]]
[[[[1,1],7],5],[[6,[5,6]],[6,[7,5]]]]
[[[[3,2],5],[[5,3],1]],[[[0,4],[9,6]],9]]
[[6,[7,6]],9]
[[[[4,0],[0,1]],7],1]
[[[[1,3],4],6],[[1,[4,2]],[1,4]]]
[[[[6,9],[4,1]],[[6,3],[0,8]]],[[4,0],[[3,2],[2,9]]]]
[[[3,6],[[2,0],[3,2]]],[2,5]]
[[[[4,3],5],5],[[4,[4,0]],6]]
[[[[4,0],3],[[3,5],8]],[[8,[4,4]],[[9,9],[4,1]]]]
[[[2,7],6],1]
[[[[5,3],[8,4]],[0,0]],4]
[[[0,[8,1]],0],3]
[[[6,5],[8,2]],[[[6,9],[6,1]],[9,9]]]
[0,[[4,9],6]]
[[9,[[9,9],4]],[[[4,7],1],2]]
[[8,0],[[[0,7],6],[[6,4],2]]]
[[1,[[2,4],8]],1]
[[[[1,3],4],[[1,3],0]],[[[1,2],3],2]]
[[[[2,1],2],[5,[2,8]]],[2,[[6,0],2]]]
[[[8,[1,0]],[[6,7],[9,6]]],[[2,[9,7]],5]]
[[[3,[2,0]],[[3,2],[0,0]]],[[[4,6],[9,4]],[[7,8],[5,1]]]]
[[3,[[9,9],[7,2]]],[[1,3],[2,[3,2]]]]
[4,[4,[[9,5],6]]]
[[[[5,7],7],[[3,4],0]],[[9,[8,2]],[2,3]]]
[[[[2,1],[5,7]],4],[[[6,3],8],[[1,6],[5,1]]]]
[[[4,4],[[0,9],[7,8]]],[[2,[2,5]],5]]
[1,[5,[[3,7],[8,2]]]]
[[[[9,5],[8,6]],[5,5]],[[[9,2],8],[[9,3],[3,8]]]]
[0,[[9,5],[[3,7],7]]]
[[[8,[0,4]],[[2,9],6]],[[6,[8,0]],4]]
[[0,[3,5]],[[5,[0,1]],[[3,6],7]]]
[[2,[7,1]],[[[5,0],[7,7]],[[2,3],9]]]
[[5,[9,[3,9]]],[[8,[3,7]],[[7,6],[3,0]]]]
[[[4,[2,5]],5],[3,1]]
[[[[4,3],1],[[5,7],6]],[0,[3,1]]]
[[8,9],[[[0,7],5],[6,[5,7]]]]
[[6,8],[[5,8],[[8,2],[6,0]]]]
[[1,[5,6]],5]
[[[6,1],[9,[1,2]]],1]
[[5,[7,[4,8]]],[[4,[2,9]],5]]
[[[2,2],[[7,1],3]],[[[9,7],[4,6]],[1,[0,1]]]]
[[3,[6,[4,5]]],2]
[[[0,2],[[8,1],[0,6]]],[[7,[9,6]],0]]
[[[[1,0],[5,1]],[[0,6],5]],[[[1,8],8],[[0,2],5]]]
[[6,[[3,6],6]],[[[9,7],[6,4]],[[9,5],1]]]
[[[0,[5,6]],[9,0]],[[2,9],9]]
[1,[[4,[9,3]],0]]
[[1,0],[[1,9],[4,8]]]
[[[9,3],[7,0]],[[[5,1],[3,8]],9]]
[[[3,9],[[5,9],2]],[[7,2],1]]
[[1,[[3,0],[7,6]]],[7,[8,1]]]
[0,[6,[[7,1],[1,1]]]]
[[4,[[5,0],[2,1]]],[[[8,8],[8,1]],7]]
[[[[9,3],[4,3]],4],[7,5]]
[[9,[[7,4],[8,3]]],[[[1,9],7],[[1,6],[3,1]]]]
[[6,9],[5,[0,[5,1]]]]
[[[8,7],3],[[4,8],[0,7]]]
[[[[3,1],2],[[1,6],[4,3]]],[0,6]]
[[5,[[5,4],3]],[[8,8],9]]
[[5,[3,[4,5]]],[[2,[6,0]],[6,1]]]
[[[[9,5],3],6],[[8,[1,9]],[[5,2],5]]]
[[[7,5],[[3,6],4]],[6,[[5,1],[0,1]]]]
[[1,[[4,8],[1,3]]],7]
[[4,[[4,0],5]],[[[6,2],7],[[4,8],[4,9]]]]
[[[[2,3],[0,9]],[7,2]],[4,5]]
[[[[7,7],[8,0]],[7,7]],[[[6,6],[3,2]],[4,[4,3]]]]
[[[[8,7],6],[[5,5],0]],[[6,[7,3]],[[4,1],[1,7]]]]
[[[2,[2,2]],[[5,2],1]],[[9,[9,2]],6]]
[[[[1,7],6],[[8,8],5]],[6,[1,[1,7]]]]
[[[[8,6],[3,2]],[[5,2],[2,0]]],[[[8,7],2],[[5,5],2]]]
[[[8,[9,0]],[[9,5],[7,5]]],[[5,1],[[1,1],[4,6]]]]
[5,[9,[[0,2],7]]]
[8,[[0,[4,9]],[[7,4],9]]]
[[[[2,9],5],[[0,6],[6,6]]],[[0,6],[[4,2],[9,9]]]]
[7,[[[4,3],3],[[5,4],[6,0]]]]
[[0,[8,[1,1]]],5]
[[[1,8],[[4,6],[9,7]]],[[[6,6],[2,6]],[4,3]]]
[[0,[[7,5],[9,9]]],[[9,7],[6,2]]]
[[[9,[3,0]],[[1,4],0]],[[1,1],1]]
[[[0,7],[[3,0],8]],[[6,[8,0]],[[4,5],[4,0]]]]
[[[[2,9],[4,2]],[5,[9,3]]],[4,[2,[3,4]]]]
[[[1,[7,3]],[[5,7],0]],[6,[[6,5],2]]]
[4,5]
[[7,9],[6,[[6,5],[1,0]]]]
[[4,[[7,5],8]],[[4,0],[[6,6],[0,4]]]]
[[[9,[7,7]],[[4,2],7]],4]
[[0,[0,3]],5]";
