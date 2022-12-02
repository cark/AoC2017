const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let part1 = part1(INPUT);
    let part1_dur = setup_time.elapsed().as_micros();
    let setup_time = std::time::Instant::now();
    let part2 = part2(INPUT);
    let part2_dur = setup_time.elapsed().as_micros();
    println!("Part1 : {} in {} µs", part1, part1_dur);
    println!("Part2 : {} in {} µs", part2, part2_dur);
}

fn part1(input: &str) -> i32 {
    solve(input, checksum)
}

fn part2(input: &str) -> i32 {
    solve(input, evenly_divisible_checksum)
}

fn solve(input: &str, line_val: impl Fn(&str) -> i32) -> i32 {
    input.trim().lines().map(line_val).sum()
}

fn checksum(line: &str) -> i32 {
    let (min, max) = values(line).fold((i32::MAX, i32::MIN), |(min, max), value| {
        (min.min(value), max.max(value))
    });
    max - min
}

fn values(line: &str) -> impl Iterator<Item = i32> + Clone + '_ {
    line.split_whitespace().map(|s| s.parse::<i32>().unwrap())
}

fn evenly_divisible_checksum(line: &str) -> i32 {
    let result = combinations(line).find(|(v1, v2)| v1 % v2 == 0).unwrap();
    result.0 / result.1
}

fn combinations(line: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    let vals = values(line).collect::<Vec<i32>>().into_iter();
    vals.clone()
        .enumerate()
        .map(move |(i, value1)| {
            vals.clone()
                .enumerate()
                .filter_map(move |(j, value2)| (i != j).then(|| (value1, value2)))
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT2), 9);
    }
}
