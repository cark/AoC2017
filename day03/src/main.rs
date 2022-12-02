use std::collections::HashMap;

// There must be a mathy way to do this, but i sure am too dumb for that.
// So brute force it is ! weeee...

fn main() {
    let setup_time = std::time::Instant::now();
    let part1 = part1(277678);
    let part1_dur = setup_time.elapsed().as_micros();
    let setup_time = std::time::Instant::now();
    let part2 = part2(277678);
    let part2_dur = setup_time.elapsed().as_micros();
    println!("Part1 : {} in {} µs", part1, part1_dur);
    println!("Part2 : {} in {} µs", part2, part2_dur);
}

type Coord = (i32, i32);
const DIRS: [Coord; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/// produces an iterator that successively returns 1, 1, 2, 2, 3, 3, 4, 4, 5, 5 ...
fn repeated_naturals() -> impl Iterator<Item = usize> {
    (1usize..).map(|i| std::iter::repeat(i).take(2)).flatten()
}

/// produces an iterator that successively returns 0, 1, 2, 2, 3, 3, 4, 4, 4, 5, 5, 5 ...
fn indexes() -> impl Iterator<Item = usize> {
    repeated_naturals()
        .zip(0usize..)
        .map(|(repeats, nat)| std::iter::repeat(nat).take(repeats))
        .flatten()
}

fn add_coords((x1, y1): Coord, (x2, y2): Coord) -> Coord {
    (x1 + x2, y1 + y2)
}

/// produces an iterator that will return the successive coordinates
/// of our spiral (0,0), (1,0), (1,1), (0,1), (-1,1) ...
fn coord_iter() -> impl Iterator<Item = Coord> {
    let mut coord = (0, 0);
    std::iter::once((0, 0)).chain(indexes().map(move |i| {
        coord = add_coords(DIRS[i % 4], coord);
        coord
    }))
}

fn square_coord(square: usize) -> Coord {
    // Knuth was wrong, the actual root of all evil is 1 based addressing
    coord_iter().nth(square - 1).unwrap()
}

fn part1(square: usize) -> i32 {
    let coord = square_coord(square);
    coord.0.abs() + coord.1.abs()
}

/// answers the age old question: what's around this coordinate ?
fn coords_around(coord: Coord) -> impl Iterator<Item = Coord> {
    (-1i32..=1)
        .map(|x| (-1i32..=1).map(move |y| (x, y)))
        .flatten()
        .filter(|(x, y)| !(*x == 0 && *y == 0))
        .map(move |c| add_coords(c, coord))
}

fn part2(input: i32) -> i32 {
    let mut hm = HashMap::new();
    hm.insert((0, 0), 1);
    // we just added the first iteration, so skip it
    for coord in coord_iter().skip(1) {
        let val = coords_around(coord)
            .map(|val| *hm.get(&val).unwrap_or(&0))
            .sum();
        hm.insert(coord, val);
        if val > input {
            return val;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_naturals() {
        let i = repeated_naturals();
        let result = [1usize, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        for (i, r) in i.zip(result.into_iter()) {
            assert_eq!(i, r);
        }
    }

    #[test]
    fn test_indexes() {
        let i = indexes();
        let result = [0usize, 1, 2, 2, 3, 3, 4, 4, 4, 5, 5, 5];
        for (i, r) in i.zip(result.into_iter()) {
            assert_eq!(i, r);
        }
    }

    #[test]
    fn test_square_coord() {
        assert_eq!(square_coord(1), (0, 0));
        assert_eq!(square_coord(2), (1, 0));
        assert_eq!(square_coord(3), (1, 1));
        assert_eq!(square_coord(13), (2, 2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(1), 0);
        assert_eq!(part1(12), 3);
        assert_eq!(part1(23), 2);
        assert_eq!(part1(1024), 31);
    }
}
