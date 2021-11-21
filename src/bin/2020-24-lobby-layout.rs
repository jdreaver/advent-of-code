use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

fn main() {
    let mut black_tiles = flip_tiles(INPUT);
    println!("part1 {}", black_tiles.len());

    for _ in 1..=100 {
        black_tiles = art_exhibit_day(black_tiles);
    }
    println!("part2 {}", black_tiles.len());
}

fn flip_tiles(inputs: &str) -> HashSet<HexCoord> {
    let mut black_tiles: HashSet<HexCoord> = HashSet::new();
    for coord in inputs.lines().map(final_coords) {
        if black_tiles.contains(&coord) {
            black_tiles.remove(&coord);
        } else {
            black_tiles.insert(coord);
        }
    }

    black_tiles
}

#[allow(clippy::unnecessary_fold)] // This doesn't work???
fn final_coords(input: &str) -> HexCoord {
    parse_directions(input)
        .iter()
        .map(direction_coord)
        .fold(HexCoord { q: 0, r: 0 }, |acc, x| acc + x)
}

fn art_exhibit_day(black_tiles: HashSet<HexCoord>) -> HashSet<HexCoord> {
    // First count how many black neighbors each tile has
    let mut black_neighbors: HashMap<HexCoord, u8> = HashMap::new();
    for black_tile in black_tiles.iter() {
        for neighbor in neighbors(black_tile) {
            *black_neighbors.entry(neighbor).or_insert(0) += 1;
        }
    }

    // Insert all the black tiles also (they might not have neighbors)
    // (We actually don't need this because these will all turn white)
    // for black_tile in black_tiles.iter() {
    //     black_neighbors.entry(black_tile.clone()).or_insert(0);
    // }

    // Iterate over all of the tiles now and flip
    let mut output: HashSet<HexCoord> = HashSet::new();
    for (tile, &count) in black_neighbors.iter() {
        // Any black tile with zero or more than 2 black tiles
        // immediately adjacent to it is flipped to white.
        //
        // In this case, if the tile has 1 or 2 neighbors, keep it
        // black and put it in the output.
        let keep_black = black_tiles.contains(tile) && (count == 1 || count == 2);

        // Any white tile with exactly 2 black tiles immediately adjacent
        // to it is flipped to black.
        let flip_white = !black_tiles.contains(tile) && count == 2;

        if keep_black || flip_white {
            output.insert(tile.clone());
        }
    }

    output
}

// Hex coords are 3D using x, y, and z
// See https://www.redblobgames.com/grids/hexagons/
//        -r
//         |
//        /\
//       /  \ +q
//      /    \/
//      |    |
//      |    |
//     /\    /
//   -q  \  /
//        \/
//         |
//        +r

fn direction_coord(dir: &Direction) -> HexCoord {
    match dir {
        Direction::East => HexCoord { q: 1, r: 0 },
        Direction::Southeast => HexCoord { q: 0, r: 1 },
        Direction::Southwest => HexCoord { q: -1, r: 1 },
        Direction::West => HexCoord { q: -1, r: 0 },
        Direction::Northwest => HexCoord { q: 0, r: -1 },
        Direction::Northeast => HexCoord { q: 1, r: -1 },
    }
}

fn neighbors(coord: &HexCoord) -> Vec<HexCoord> {
    vec![
        coord + &direction_coord(&Direction::East),
        coord + &direction_coord(&Direction::Southeast),
        coord + &direction_coord(&Direction::Southwest),
        coord + &direction_coord(&Direction::West),
        coord + &direction_coord(&Direction::Northwest),
        coord + &direction_coord(&Direction::Northeast),
    ]
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct HexCoord {
    q: i32,
    r: i32,
}

impl Add for HexCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl<'a, 'b> Add<&'b HexCoord> for &'a HexCoord {
    type Output = HexCoord;

    fn add(self, other: &'b HexCoord) -> HexCoord {
        HexCoord {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl Sub for HexCoord {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            q: self.q - other.q,
            r: self.r - other.r,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

fn parse_directions(input: &str) -> Vec<Direction> {
    let chars = input.chars().collect::<Vec<char>>();

    let mut directions = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            'e' => {
                directions.push(Direction::East);
                i += 1;
            }
            'w' => {
                directions.push(Direction::West);
                i += 1;
            }
            's' => match chars[i + 1] {
                'e' => {
                    directions.push(Direction::Southeast);
                    i += 2;
                }
                'w' => {
                    directions.push(Direction::Southwest);
                    i += 2;
                }
                _ => panic!("failed to parse {}{}", chars[i], chars[i + 1]),
            },
            'n' => match chars[i + 1] {
                'e' => {
                    directions.push(Direction::Northeast);
                    i += 2;
                }
                'w' => {
                    directions.push(Direction::Northwest);
                    i += 2;
                }
                _ => panic!("failed to parse {}{}", chars[i], chars[i + 1]),
            },
            _ => panic!("failed to parse {}", chars[i]),
        };
    }
    directions
}

#[test]
fn test_parse_directions() {
    assert_eq!(
        parse_directions("esewswnenwe"),
        vec![
            Direction::East,
            Direction::Southeast,
            Direction::West,
            Direction::Southwest,
            Direction::Northeast,
            Direction::Northwest,
            Direction::East,
        ]
    );
}

const _EXAMPLE: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

const INPUT: &str = "swsenenwneswnewseswwseswnwsweeswnw
esweeeeneeeneeeweeeenenenee
ewewsewswnewnwnewwwwsew
nwnwnwnwnenwwnwsenwnwnwnwnwnwnw
nwswseswneswseswswneswswseneseswswsenwswse
swswswneswswswswwneseswwswsw
newnwwnwnenenenenwsweenenwnenwnenese
senwsewseneneseneenenwwneneeswnewsw
eeeeneweeeseeeeeenewneswe
wnwswwewwwsewnwnwwnwnwwwnwnwnww
nenwnwnwnesenwwnwnwwneswnwnenesweneenwnw
nwwnwwwwenwwswnwwwwwewsesew
swnwswwswswewswseswnwne
wswnewswnwwwwenewsewwwwneswe
nwnwnenenewneneneneneenenesenwneswneswse
nwewsenwnwneseswwwwwwswwswwwwe
nesenwseneswseswnwneseseseseseswnwse
wnesweseswenwnenwnweseeseseeswnwse
swswnenenenenewneneeeneneeneneenenene
seseseseeweseeesee
wwwwwwwwwnewnwwwswww
sesenwesenwseeseseseewswneseseesesw
wnewwwsewnwwnwnwewnwswnwwwwnwe
nwnwnenwnwneneenewswnwnenwswnwnesenwsw
eeswwneeneneeneeneeeeesw
seneenesewneeneswwnenenweeenenewwe
seswseswsenwswswseswswswswenwswsesesw
nwnwnenwnenwnenwnwnwnenwswnenwwwnwese
wnwwwwwnwwwnwwnwnwseneenwnwnww
ewnwnwnwnwnwnenwnwnwnenenwsenenwnwnene
neneweeeeneeneesweseswswneseswse
seswseeneswswswseseewsewnwswswwnwne
swwneneneweseswnenwneeneneeeeeeee
swwwwwwwwwwwwswswwswswnesew
eewenwseeeeeweeeeneeese
neneneneneeneswneeenee
sewnwnwwswnwseenwwnenenwwnw
swsweswswswswsewseneseswswsesweswnwsw
nenewewwwwwwwwwnewwswsesewww
enwsweneseseeesese
swneseseseneeseesenwwseneswseseseseew
nwswswneeeewneseenwnwnesewwswnwesw
nenenwneenwneneneneneneneneneswenewne
nwneneseeenwnwnwneneneswneswwnenwnwnenw
neseenenwneeneneneeeeneene
sesesewseseseswneseesesesesenewseswsese
senwnewseneneeeneeeeeenesenwnene
eeweneneeenweeeseesweeeee
esesenwswnwseewseswseeeseswenenwse
neeneeeseswseeesweneeseeesweee
swweswnwneswswwswswneswswswswswsw
wwswwwnwnwnenewnwenwswnwsesenwnwe
neseneenwneeeeeneneeeewneeene
swsenwsesesweswsewswswswsenwneseswsesw
wewwwwwnwwwwwnwwwnw
swseweeenwswneseseesenese
senwsesesesesesesewseesewe
sewnwnwnweswnwnwsenwnwnwnwsenwnwnwnwnw
swswswswswswswswswseswswswswswswwswnesw
wswnwnwsewwnwewwewwnesewwne
enenenwnewnenwnenenwwneswneseswnenwsenwnw
seseseswswseswsewnwsewswswseeseeswse
nwseswnweswnwneseweswsesweseseswne
seseswwnwswneneseneneseeseesenwswnwww
wswswwwwswwneswseswnwswswsew
senwnwwnwnwesewswnewswwsenwwnwnwwnw
swsewneswswswswwswswwswswswsw
eneseeeswneswneeseesenwewsenwewnwnw
neneneeeneweneneeswe
neeeseeenwnweewswnewneseneenenee
senewesesenweseseeesewsesesesesesee
eseeeseseseenwseeneseseeewse
nwnwseseenwenwnenwwswnwnwnwwnwnwnwnwnw
senewweseeenwsesesenesewsesesesee
enenenwswnenewswenwnwneenwwnwswnenene
eswseseneswsenwswseseseswsesesewsenwsesee
wnwneswneseneneneneeneneswenenenenewne
seswneewsenesesewnwwswnesewswesenwne
eswwesenwwenwnwnweseseeeeeese
neeneswneeswnenesenwnwneswnenwneneswsene
swswneseseswswswswwewnwsenwesweneswnwe
nwenwwwnwnwnenwnenwnwnwnwnwsweneswe
seseneneeseseeeeswsenwwwesesenwe
sweswseswwnwsesesewseseseseswseneswnene
seswseseseseseswwseswsenesesesesesesese
sesesenwswswseswswsw
senwseeeeseseeseseseenwewswesee
wwsenwewwnewwswwwwwswwwwwne
nwnwnwneswnwnwnwsenwwwnwwnwnwnwenwse
swwwswswswwwsewnenwswwswwswseww
enweenesesesenewnwnwwswswswseseese
swnenenenenenenenwnenesenewnenwnenenwnene
enenesweeeeseneeneeeneeneeew
wnweseeswnwwenwnenwnwnwwenwnwsenwwnw
neswswswseswswswwswnesw
neswseeneseswsewseseswseseswswwswswse
seeneswsesenweeeseeseswsewenesee
wewewwnenesewsenwwnenwnwseswsww
nwnwsesewnwseeeswnwnwseewseenw
nwwwwwnwwnwnwwwwnwsenwwwwsew
eenwneseeeneeneenenwseeeeeee
nwenwnwnenwswnwneesenwwwnwnwnwsenwnw
swswswswseswswswswwseswswswswswswswswne
swwwwwwwwwwnwwewwwnwwww
esweeeeeeneeenwnwenesw
eeenweeeseese
wwwnewwsewewwwwwwwwww
seseseeseseesesenwesewseeseseseswnesese
senwnwwneswwswwnenwwwewnwnwwnww
swnwsesewneenwenwseenwneeseeseewsw
eneeneneneneneneswsewswsenenenwnenenenw
wneesweseswswwsewswnweswswewswswse
neseneneeewneneenewnenenenenenenenene
wswnesewwsenewswswswswneswswswsweesw
swswsesenwseswsewseswneseseseswsesw
seswwswwwseswswswswswweswneswswnewsww
neneesenewewnenwneneneswnwweswswnese
swswswwwswswswswswwneswswwwswseww
neenenenenenenenenenwsewneneneneswnenene
eneeeeseenenwwesenesweeseeesw
swswswswswswwswnwswswwswseswswswsw
eneseseseswsesesewsesesesesenwseseswenese
nenesweenenenewneneeeneswnweenene
eneswsenwseenwenwene
swsewnenwenwwwwseseseswnwwne
seeseseseseseseswsewsesesenwseseseewse
swenewswnenwswsweswneswswswse
wnwwnwwwswwwwnwsenenwnwwwnwwww
eswswwswswnwswswwwswswsewsweeswww
nwwnwenwnwnwwnwsewnwwwnwnenwnwnwnwnw
seswwseeswswseenwneseswnewwswseswsw
enewwwswesewwnenwseswseweenwnwnew
swseswseesenwseseseseseseseswseswwsee
neneneneneneneneneneneenwnewneswe
wnwswswnewnwnwwwsenewweeswnwnenww
swenweenweeneswneeeeneneee
nesweswswseeswswsenwswsewwseneneswse
nwneswewnwnenwnwnenesenenenwnwnwneswnene
wenwswnwnwnwnwnenwnwnwenenwnwnwnwnwnwnwne
seseneesewnwnwneenwseeswswsesewnese
swnewswnwswnwnwnwwenwnwseewnwnenenww
seesesewenwneeeenwnesenenwsweseswnwsw
swnenenwnwnwnenenwnwnwnwswnenenenenwnwse
ewewnwwswesewsweswnweswswnwnwwnesw
neseneewwswwwwnwwwewwwwwsww
enwnwnwwswnesewwnwnwsewswnw
sewwwwwsweswnwswswnwwwwewsesw
neneneseweneenenwesewnenenenenenese
eeeenweeeeswseewnweewneswnew
wnwnenwnweneneswnenwenenenwnesenwne
eseseseseswsenwseseswsesesesesesesesese
eseeseeseeesesenwwseene
nenwnwnwnewnwnesenwnwsenwnwneneseesw
ewwnwnewnwwnwnwnwnwnwswswswwwnwenw
senwwneeneeeneneneenewneswenwnene
nwnenwnenwewenwnenewseeeenwswswwswe
wwewwwwnwwnwwwnw
eeeeeeseseeeeeeeeeew
wnwneeeseeneeneeneeneweseeenesw
nwwnwsweswwwseww
swswwswneswseswswswseswswseswsesweswwsw
newneneeneseenenenenenenenenenenewwne
nesewwwwswwnwwwnwwwneewwwsee
enwseseseseeseseseeeseesese
seeseseseseseeseseswneseswseseesesesenwse
nwseeseeesesewswseswwsenwseswneseesw
wneseenwnenwenwneeneneneneneeseesee
nwneesenwsweeneneneeseeneesenwww
nenenwswnwswnwnwnwnwnwnenwnenenwnenwnwnwnw
nwnwnwenwnwnwnwnwnwnwnwnewnenwnwswnwnw
wwwwnwwnwwnwewnwnwnwwnwwwswnw
neewneenewseneneeneneneneneneneene
neneneneneneneneneneswnenwnenenwnesenwnene
eeeeeeeseeeeeeeneeweee
ewwsweswsweseswswswswnwnwewnwnesw
weswswseseenweneesenenwnwnwneswswse
nwswswseswwswswswswwwswsww
seswseswwseseswswswneseswseswswswneseswswsw
eneeneewnenwneeseeneneeneneneseeee
eenwnweswenwenenwswswneeeewese
nenenenesweneswneswnenwne
enweseeseeneesweswnwsee
swnesesenenwwswnwsesenweneenenweswwew
nenwnwnwnwnwnwnwnwnwwnwnesenwswnwenwnw
nwwnenwnwnwswnwswsenwnwenwnwswnwenwnwe
swweeenwneswnwnwwswsewwswswseneew
esewnwseenwseseenweseseseesesesenwse
enweseeeeseseeneeseeeseweee
eeseeeeseseseeeewneeeseeeswe
swswwwnewswwswwswwswwswsw
eewseeenwsweenweneesewnweew
wswswsenweswnwsenwnwswswwneseweneww
nenewswnenesenenenenenwnwnenewneenenene
nwnwnwnwnwenenwnwnwwnwnwnw
sweweesweseeeswnweenwseneeenwee
ewesesenweseenwseseseeseneseeeswe
eneenenewnenenenenesenenene
eeeesesweeewseseeeneesewenesew
wswwwswswswwswswnwsweswswwswnewseswsw
swswswseswswswswswswswswswseseseswswenw
neenenwenenweseweneenesenwswneswnwswne
seswwwwwwwwwwwwwwwwnesenewne
swswswswswsweswswseswswswswswswseenwnwswsw
eneeeseeseseeseeseewseseseseee
swneswswwnewewswswsewwswwswnewse
wnwnwwwswwnwwewwwwwseewnewsw
senweeneeseeewenwsew
nwnwnwnwseswnwnwnwnwenwnwenwnwnwnwwwwsw
eeseneeeswewenwneeseeeeeee
wnwenwwnwnwnwnwe
wswwswswwwwwwsewswwneswswswsww
nwnwnwnwnenwnwnenenesw
nenewwnwnwswnwesenwneseneenenwneewne
nwneneswnwnenenwnwwsenwnenwnenenwnenenw
swwwwswswnenwsweswenwswseneswwswnese
eeeeewneweeeeswsewesweenwene
seswsewnwwnenwwsesenesewnwweeswswnwse
senweeseseswnewseseswsenesese
nwsenwnwnwnwnwnwenwnwnwnwwnwwwnwww
wseseseseseswswswswneswse
nwneneswnenenenenwnenwne
esenwseeswseseseseseseswsesenesenwswse
seswswswswnenwswenewenwneswseswwswswswse
sweswwwnwswwswwnweswww
swwswsewswwswwswwswswwnewneww
swseneswwnwweswswseswseeswwneswswswew
eseseseesesesenweeseseseweeseswese
swswnwnwnweswswswswswswswee
swseneswsesenesweewwseneswseswswnwnw
sewseeseseseeeseseesesesenese
seswnesesesesesesewseseseswsesesesesese
seseseseswseswseseseseswsesewseswsenese
enwnenwenwnenwnwswsenwnwswne
enenewnenwneneneneneneneewnenenenene
nwwsweneswwswwsewswwswwswwenwswsw
swsewseseneswswswseseswseeswseswsesese
neneseewneseneneneeneneenwnenewe
wwnwwwewswewwwwwwwswwswww
eeswneneneneneneneeneneneswswnenenwnw
nwnwnenenwnwsenwnwnenenwnenw
eeseseseesenwseseeesesesewseseseseesw
wewwwswwwswwwwwwswnwsewwnw
nwwnwnwwwnwnwewnwnwwnwswswnwwnwe
nenwnenwnwnenwnwnwswnwnwwnwnesenwnweswnenw
wwneweenenwswnesesewnewswswwwse
seswswsesesesewswseseseseseseneseswsese
swswseswseseswseswseneeswswseswsesewse
eneneenweeewnwesenenwseswseewwe
swenenweeeneeeneewseeswenwee
enesenewwwswseseswsese
eswneseweneneeneneeenweneneeee
eseseeseseeweswswenwneeeeneewne
wnwwneenwwwwsenwwwwnwwwwww
nwnweseenweewneswesweeeeeneew
swswswwnewswwswseneswwwswwwswsww
nenwswnwnwnwenwnwnwnenenwnwnenwnwnwnene
nwnwwwwnwnwwnwnewnwnwnwnwsenwwww
wswnwwseswwswwswwnwwewswsewswew
wneeweneneneseneswnewnenesesenenwnee
sesesesenwseswseswsesesesenwsewseneesese
swseswsenwsweseswsewswsweswse
nesesesweewwsweswseseswnwnwnwswsee
nenwnenwnwnenenwneswseseenw
sesenwswsenenwseswnew
sewseseseseeseseseseseesesesesenwseseswse
sewwenwnwnwwnenwnewswnwwwwnwwwnww
nwnenesewnwnwnwnwnwnenwsenwnwnwnwnwnwnwnw
nenwseneswnweseseswneweswnesenweewne
sesenwseseseewseseewseneseesesesesw
wwnwwnwnwwseswwnwenwnwwwnwnwewww
seseswsenwseswswseswwneseswsese
sweeswnenenenwneneenweeeneeeeene
seseseseswneswseswseswsenwseseseswsese
enenenenenenwnwnwnwnenesenenwnwneswnwnene
swwsweswwswwwswswswsw
wwswwnwsewswwnenwwwwwnwenwnww
eenwnwseseseseeeeeeeeeweeswswe
nwsenwnwnwnwnwnwnwnwnenwnwnenwwne
swwswswsweswswswswwesweseswswswswsww
seseesenwseweseeeseseewseseswsese
nwseeeesweeeeeseeweeenweeee
seeswneseseseseseswsesesesesesesesenw
eswenewsenwwneeeneeneeneenenee
wswenwnwenwwnenwwswesewseneweewsw
sweswenwswnwswswnwnwwnweenenwnwe
nwenweeswswwww
nwneneneswnwnenenenwnwneneswnwnwnesenenene
swseeesweeewsesenwweseenweenee
nenwneswneswneneenesenesewnwnesenewnenene
sewsesesesesesesenwseseeseneseewwsese
seswswswswwswwneswwswswswswwswwwnw
eeesenweeewseweeeswweneswnenw
eeeeeneeeweseeweeeeeeee
seswnesesewwseseeenesenewseswwnesw
eeeeeeseeewseeeeeneseeee
nwwwnwenwwswwnwenwwsenesenwwnw
wwwwwwwwwsenwwnwwnwwnwwwsew
seswswswseswseseswswswseswnwesenwseswsesw
eneeneeneneeeeweneneneseweneee
swwwwewswnwswewnwswwswwswwwswsw
eswneseeneneewneswseeweeeneenwnenw
swswnwwwseswnwnesewseswwnwswsenwswe
swswswwsewnwwwewswwneswsewwwnw
nwnwwnwnwwnwnwsenwnwnwnwnwnwseenwnwnwnw
seseseewsesesesesesenenewsesesesesesesese
nesenenenenenwnwwnwnenwnenenewneneneene
wesesesenweseseseswseseswsenwswsesewse
eseeswseeeswenweeseseeeneeee
neswwnwwwwnwwwewwnwswwenwww
nwswnwnenwnwnwnwnwnwnweswne
nwwwsenwnwnwwnwsenenwswnenwwnwnewwse
seenwseeenwweseeeweesweeeese
swswsweswswswneswswseswswwwswswswswneswne
ewwwswwwwswwwwnwwwwwswww
seswsenwseseseswsesesese
neneeswewnenwswneneseswenwseewenene
swnwnwenenenwnwnwnwseenwnwsenenwnwwnw
nwneswneneneneneneseneneswnenenwnenw
nwnwnwnwnwnwsewnwnwseenwnwnwnenwnwswnww
seswseseswseseswneswneswswseswswseswswsese
wnwnwnwnwnwnwnwnenwnwnwnwnwnwsesenwnwnwnw
seseseeesesewsenenwseweeseesesee
nwnenwwweseswnwwwnewwswseneseww
swnwseseseesesesesesewseneseseseseswsesw
seseseseesewneseseseneseeeseseeswsese
nwnwwnwnenwewwnwneseswwwwnwww
esesesesewseseseseseeseesenwsesesese
eswswnwswswneswswswseweseseswswswswne
swneenenenenenwnenenenenenwnenwnesenene
nwwswnwswswnwnenweeenenesenwwsenesw
eeeeeeeeeenweeeeenwswswee
nwnenenenwswnenenwnenenenw
nwwwnwnwenwnwnwsenwwwnwwnwwnwnwnwnw
ewneweseeseseseseese
nwnenwnwnwnewsesenwnw
nenenwnewnwnwnenwsenwswneenese
wnwwwwwnwwwwnewsewwwnwwswwnenw
enweseeeeseseeeee
wesweseneswsewnewwneewenenenwnwse
eseeswseeseeseseseseeseenweeee
wnwnweweswsesenweseseseseseswewsese
eeeneeseeenwseeeneenwseweeee
esenwseswseswswseswseseseswseseseswswse
wwseswnewwwswewwwswnewwwew
eeeneeeeeeenweeseenweeswee
sesenwneewswweneswnewsenenwenwswnw
nwnwneseenenewewnenewswneesenenene
swweswswwswswwnwwwswswwwswnwwesww
swnenenenwneneenwnwnwnenesweswnwnenenwnw
nenwnwwewwnwnwnwsewswneenwswswnwnw
nwwwsewwnwnewnesewwwwnwnwnwnww
wswwnwsenwswnweswweswswwwwenenw
neswswswneseswwswwwsewswwswnwnwneesw
newewwswwswwewwwwwwwwseswne
neenenweesweneneeneneneneneneneenese
eseseseeswseweenweseseeesesesesee
swseswseswswswseswseswswswnwsw
nenenenenenesewneewsenenenenenenenenene
eeswneseenweseeeeeeseewsewseee
nwewswswwswswwswswwswswswswswswnwse
nwnwnwnwnwnenwnwnwnenwswnwnwnw
eeeseeeeeeweenweseseseeese
swswwwswnewsewswwwnwswswsenwswsesw
seseswsesewsenesenesenw
seseswswseneswswswneswseswswnwsesesesesese
nwnwwnwnwnwenwnwnwnwnwnwnwnwnwnwnwnw
neeneseeneneneneeneenewneneneswnenew
nwsenwnwnwnwwnwsenwnwnwnwnwenwnwnw
swswswnwswswswswneswneswswswswewseswnwsw
seswneswswswswswswswseswswswswswswenwsw
wwwwnewnewnewswwswwswwwseswww
swenweswseswseneseswswsesewsese
ewswseswwsewsweswwswwswsenenenesene
swwnwnwnwseneseenenenwneswwwwwseswse
nesweneneeseneneneneneenenenenenwnwnenee
enwnwwwnwnwnwnwnwnwnwnenwsenwnwwwnwnw
wwwwwswwwwwnwwwewwnew
nenenesenenwnwswenwnwnenenwneneneneeswne
eeneeneneesweeneneneneeeeewene
eeseseenwnwsesesenwesenwesesesewese
neeeeswswneeeeeeenweeene
seseesewnesewseseeseseswseneseswnwsene
sesesewseeeneeseeseseseseeseeese
wswwnwwnewsewnwnwnwesewwnwnesene
eseeneenenewnenenwseesenwnwnesewnene
eseeneeswnwneesesewwneswewwneesw
neneswseneenwnenenenewneneneneneneseneene
sesesweseswsesesewseswsesewsesweswse
sewsesesesesewseseseneseeseneesesese
swwwenwwnwswnwnwneswwwnenwsewnwww
swwswswseseswswnwseswswswswsweseswsesw
senwenwnewweneneswswnwswwsenwenwenwnw
nenwnenenenenenenenenwneneneneneneenwwsw
wwnwwsewsewswnewsenwneesw
nenwnwseswnwnwenwesewnwneeswswnwnwswnw
nenwswnenenenenweneseneewseeneewswene
esenwwsenenenewnwnwswnwnwnwwwwnwnwnww
sesenweeeseseeeseseeseeeseswese
neneneneneenenenewsenenenenenenenenene
eeeseeeesweneneneeeeeenenwe
eswwwwnwneewwswwnwwneswwseswswwsw
wseseeeesesesesenweeneseseseseeee
esesesenweeeeeweeeswenwne
wwwwswneenwsww
seseswseneswswswseswseswswsewswswswseese
weseswseeeneseseewnweseesesesee
swswswswswswswswneswswsw
swwswswswswswswswwswswenwwswww
nweneneneeeneneeneeseneneneneene
swwswwwneneswswwswwswwseswnewswsw
eeeewenwweeeesweeeee
wnwseeewswswwwswsewwswnewnenew
ewseneeswswneswenweswnwwwwneswwe
eswseswnenwswseswwseseesesesenesesesw
senweeseseesesesesesw
nwsesesewsewnweweseseeseswsesesw
sewwwseseswswneenewenenwsenwseww
nwnenwnwneswswsenweswneneneenenenenene
eswnweeeeneeseneeeswenweneenw
wsenwnwwnwnwnwnwnwnwnwnwwnw
nwnwenesewenwnwnwwwnwwnwnwwnwew
nwsenwnenwnesenwnwnwnww
wnewneswnwwsesenwwweweseneneww
swseseseneeseswsewse
nenwsenwnwsenenwwnwwnwnenwnesenwnwnwnw
neswswswswnweewnenwswswswswswswseswswne
nwseseseseseeeseseseeeeseeeswsese
wnwnwnwwwenwsewnwnwwnwnwwnwnwnenw
wwwwwnwwewnwww
nwswswswswswswswswswswswswswneswseswneswsw
nenesweneneeneneneneneeneneenenee
swswswswswswwseswwswwswsweswswswnwwsw
nwwewswwnwwwwnewwnw
eeewenweeeseneeeseeeeeeeswe
swswenweswseseswneswswswsewsesesenenww
eeseeeeeeneswseseseeeweeeee
eeeseseeeeeeeeseenwenenweswesw
neswseswswseseseswsesw
sesenwwseneesewseneseseewneswswneswne
senenwswswwwsewwsenenwwenwewnew
neeseeweeeneeenenweeseeeee
nwswwswswswseswwswswswnewswswwswsesw
swneeneneeneswnwsesesesewnwesewsesesw
swswenwneneewneswwswswesesewsesw
nwnwsenenwnenwsenwnwnenwneneeswwnenewnwnw
swsenenewswswsewwnewwwnwsewswwsw
swenenenwseseneweenene
wswwswneswsewswswswswswswsw
wsenwsenwwwsewwnwswwnenwwnwnwnwew
nesweweeswwseseseseeeeneenwew
wseseneswnwswseseswseswswsenwwswswswesw
wwnwnwnewswenweswenwnwenwnwswnww
seswneswswseswwseswswseneswswnweswswswsesw
swwwnewswwswswwwwwsww
enwswswneeneneeneneneeneee
swswwswswswswswswswneswswswswswsw
sweseneswwseeeneeenwnenweswnenwne
eseeeeweneeseesewseswnewe
enwesweseenwseseeeesweeeenese
eswsweesenwenweeeeseeeeeenwee
eeeesenweseeeeeesweeseeee
wwseenenwnwwnwnwwwwnwswsenwwnwenw
sesenwseseseeseseswseswseswseswnwsenwsese
eeeeneeneswnenwee
seswneseswseswnwswswesesewseswswswswsenwse
nwnwwnwwenwnwnwnw
eenenenwneneeswneeneneseneneenenene
seeseseseswseseesesene
seseseswseseswnwswswwseswsweseswsesesenw
nenenenenenesenenwnenenenenenesenenenewnenw
nwenwnenenwnwenwswnwnwnwnwnwnwneneswsw
nwneswswnenenwnwnenwnenwnenwnwswnenwnwnw
eneneneeeeneeeweneeneenene
enenenenwnenenesenenenenewneenenenene
seseseseseswsesesesesesesesesesewnenesese
nwnwenwwnwenwwwenwnwnwnwwwnwwsw
eeseeesweeeseeeseeeeewnwenee
sweneneeneeeeneeewneneeneneenw
nenewnwneseseneneenwwswneneenenwwse
nwswwwwneswsewwswwneweweswnwsew
swswnwswswsweswsesw
nwwnwnwnenwnwnwswnwnwwnwnwnwnwwnwwew
swwwwwswswwwwneswwswsewwwwsw
seswsenenenewewnenenwnenewenenenenesee
wwnwwewnwnwenwwnwnwwnwswwwsww
wswswswswswswneswswseswswswswswswwswsw
sweseeeeeeeeenwneneeeenweee
eneneneeneneneswnenenenewenenenenwnesw
swseeesewsenwswsesesw
eeneneenenwneeneweswsweneneeene
swseseseseseseseseseeeenwswseesesesenese
eneweeneeneneseenenenewseeeneenew
eeswswseenenwnewe
ewenwwsesesesenwnwwesenenwnesesew
wwwwnwwwwnwenwwwnwwwwsenww
swnwsesewnenwsesesesene
sewnenenenwneenewsenwnenewneeesesw
swswswswswswswswseswnwswsww
seswswsenwwswseswnesweseseswswseseesenwse
eswswwswswnwseswswswswswsweswswswswsw
nwnweswwnesenwnwnwnweswnwnwnw
eswnewwswesewwwnwewwesenwnwwsww
eswnwneneneeeeswnesewnesweeeee
ewswwnwnwnwwwnwnw
eeneneeeneneneeenweneesweswnese
seswseseseswseneseseswweseseswswswseswsw
nwnwnwnwnenwwwenwenwnwnwesenwwnwnw
sewwnwwenwswnwnww
wseneseswseseswswenwseseswswswswswwse
seseeneseweneneswnwseenwswnenenenwnene
nenewswnwseseneeneneneseneneneewnenw
eneeeneeneeeeneeneweese
wwewwswwwwwwswswwww
seswseseseseseswseseneseswnesesesesese
neneswseswnwwseswswswwswswsweswnwwswe
newwswswswswwswswneeswnewswsewese
eseeeseeseeeeeseewe
enenewnenesesenewnwswsenesenenwneneneesw
neeseeeeesesweseseseeseseenweese
nweswswswswswseeneswwsewswswswswswe
wwwwwwsenwwwnwwenewwwnwwsww
swsesewnwswswswneweswswseeswswneswsew
swseswswswswswswswnesweeswswswsenwswswnw
eeeneseeneweneeeweeeneeneenee
nwnwnwnwnwenwnwnwnwswnwenwnwnwnewnwswswnw
nwwnenwwenwnwwwsenwnwnwswnwwneww
senewswswnenenenwneeneneneneneeenenenene
eeseseeseesesenweseeseeseewse
neswnesenenwswnenwswnenwswnwneswswnewseee
senwnwnenenwnwnwnenenwnwnenwnwnenene
wwnwwwwwwwwsenewnwww
nwnwnwnwwnwnenwnenwnwneneenwswenwnwse
wwwwnwsewswswwswenwswwswswwwww
nwwwwnwwswenwwwnwenwnwnwnwswnww
nenwesenwewneneseeseenwenewnenee";

// Alternate, incorrect models
//
//       +z
//  -x   _|_  +y
//    \ /   \ /
//     /     \
//     \     /
//     /\___/\
//   -y   |   +x
//        -z
//
//
//        /\
//   -z  /  \ +y
//      /    \
//      |    |
//  -x  |    | +x
//      |    |
//      \    /
//   -y  \  / +z
//        \/
//
//
//        -y
//         |
//        /\
//   +z  /  \ +x
//     \/    \/
//      |    |
//      |    |
//      |    |
//     /\    /\
//   -x  \  / -z
//        \/
//         |
//        +y
//

//
//        /\
//       /  \ -y
//      /    \
//      |    |
//  -x  |    | +x
//      |    |
//      \    /
//   +y  \  /
//        \/
