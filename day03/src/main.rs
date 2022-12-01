use std::collections::HashMap;

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

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/// produces a closure that successively returns 1, 1, 2, 2, 3, 3 ...
fn repeated_serie() -> impl FnMut() -> usize {
    let mut i = 1;
    let mut count: usize = 0;
    move || {
        let result = i;
        count += 1;
        if count == 2 {
            count = 0;
            i += 1;
        }
        result
    }
}

/// produces a closure that successively returns 0, 1, 2, 2, 3, 3, 4, 4, 4, 5, 5, 5 ...
fn index_serie() -> impl FnMut() -> usize {
    let mut serie = repeated_serie();
    let mut index = 0;
    let mut count = serie();
    move || {
        if count <= 0 {
            count = serie();
            index += 1;
        }
        count -= 1;
        index
    }
}

fn add_vec((x, y): (i32, i32), (u, v): (i32, i32)) -> (i32, i32) {
    (x + u, y + v)
}

fn square_coord(square_index: usize) -> (i32, i32) {
    let mut index = index_serie();
    let mut coord = (0, 0);
    let mut i = 1;
    while i < square_index {
        let vec = DIRS[index() % 4];
        coord = add_vec(vec, coord);
        i += 1;
    }
    coord
}

fn part1(square: usize) -> i32 {
    let coord = square_coord(square);
    coord.0.abs() + coord.1.abs()
}

fn coords_around((x, y): (i32, i32)) -> [(i32, i32); 8] {
    let mut result = [(0, 0); 8];
    let mut index = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if !(i == 0 && j == 0) {
                result[index] = (x + i, y + j);
                index += 1;
            }
        }
    }
    result
}

fn part2(input: i32) -> i32 {
    let mut index = index_serie();
    let mut coord = (0, 0);
    let mut hm = HashMap::new();
    hm.insert((0, 0), 1);
    let mut total_around = 1;
    while total_around < input {
        let vec = DIRS[index() % 4];
        coord = add_vec(vec, coord);
        total_around = coords_around(coord)
            .iter()
            .map(|val| *hm.get(val).unwrap_or(&0))
            .sum();
        hm.insert(coord, total_around);
    }
    total_around
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_serie() {
        let mut double_serie = repeated_serie();
        assert_eq!(double_serie(), 1);
        assert_eq!(double_serie(), 1);
        assert_eq!(double_serie(), 2);
        assert_eq!(double_serie(), 2);
    }

    #[test]
    fn test_index() {
        let mut index = index_serie();
        assert_eq!(index(), 0);
        assert_eq!(index(), 1);
        assert_eq!(index(), 2);
        assert_eq!(index(), 2);
        assert_eq!(index(), 3);
        assert_eq!(index(), 3);
        assert_eq!(index(), 4);
        assert_eq!(index(), 4);
        assert_eq!(index(), 4);
        assert_eq!(index(), 5);
        assert_eq!(index(), 5);
        assert_eq!(index(), 5);
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

    #[test]
    fn test_coords_around() {
        assert_eq!(
            coords_around((0, 0)),
            [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1)
            ]
        );
    }
}
