// Santa needs help mining some AdventCoins (very similar to bitcoins)
// to use as gifts for all the economically forward-thinking little
// girls and boys.
//
// To do this, he needs to find MD5 hashes which, in hexadecimal,
// start with at least five zeroes. The input to the MD5 hash is some
// secret key (your puzzle input, given below) followed by a number in
// decimal. To mine AdventCoins, you must find Santa the lowest
// positive number (no leading zeroes: 1, 2, 3, ...) that produces
// such a hash.
//
// For example:
//
//     If your secret key is abcdef, the answer is 609043, because the
//     MD5 hash of abcdef609043 starts with five zeroes
//     (000001dbbfa...), and it is the lowest such number to do so.
//
//     If your secret key is pqrstuv, the lowest number it combines
//     with to make an MD5 hash starting with five zeroes is 1048970;
//     that is, the MD5 hash of pqrstuv1048970 looks like
//     000006136ef....
//
// Your puzzle input is iwrupvqb.
//
// For part 2: 6 leading zeros.

fn main() {
    println!("part 1 {}", mine_santa_coin("iwrupvqb", false));
    println!("part 2 {}", mine_santa_coin("iwrupvqb", true));
}

/// Find the lowest integer that, when appended to the given key,
/// produces a hash that starts with 5 zeros.
fn mine_santa_coin(key: &str, part2: bool) -> u32 {
    let key_bytes: Vec<u8> = key.bytes().collect();
    let mut i = 0;
    loop {
        let i_bytes: Vec<u8> = format!("{}", i).bytes().collect();
        let full_data: Vec<u8> = key_bytes.iter().chain(&i_bytes).cloned().collect();
        let digest = md5::compute(full_data);
        if digest[0] == 0 && digest[1] == 0 {
            // 5th zero is in 3rd u8, so for part 1 we need to bit
            // shift to get it. For part 2 we can just read whole
            // byte.
            if (!part2 && (digest[2] >> 4) == 0) || (part2 && digest[2] == 0) {
                return i;
            }
        }
        i += 1;
    }
}

#[test]
fn test_mine_santa_coin() {
    assert_eq!(mine_santa_coin("abcdef", false), 609043);
}
