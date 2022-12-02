const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let part1 = part1(INPUT);
    let part1_dur = setup_time.elapsed().as_nanos();
    let setup_time = std::time::Instant::now();
    let part2 = part2(INPUT);
    let part2_dur = setup_time.elapsed().as_nanos();
    println!("Part1: {part1} in {part1_dur} ns");
    println!("Part2: {part2} in {part2_dur} ns");
}

fn part1(data: &str) -> u64 {
    solve(data, 1)
}

fn part2(data: &str) -> u64 {
    solve(data, data.len() / 2)
}

fn solve(data: &str, offset: usize) -> u64 {
    let data = data.trim().as_bytes();
    data.iter()
        .enumerate()
        .filter_map(|(i, &value)| {
            (value == data[(i + offset) % data.len()]).then(|| (value - b'0') as u64)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1("1122"), 3);
        assert_eq!(part1("1111"), 4);
        assert_eq!(part1("1234"), 0);
        assert_eq!(part1("91212129"), 9);
    }

    #[test]
    fn test2() {
        assert_eq!(part2("1212"), 6);
        assert_eq!(part2("1221"), 0);
        assert_eq!(part2("123425"), 4);
        assert_eq!(part2("123123"), 12);
        assert_eq!(part2("12131415"), 4);
    }
}
