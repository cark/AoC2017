const INPUT: &str = include_str!("input.txt");

fn main() {
    let data = parse(INPUT);
    println!("Part1 : {}", part1(&data));
    println!("Part2 : {}", part2(&data));
}

fn part1(data: &[u8]) -> u64 {
    solve(data, 1)
}

fn part2(data: &[u8]) -> u64 {
    solve(data, data.len() / 2)
}

fn solve(data: &[u8], offset: usize) -> u64 {
    let mut result = 0;
    for (i, &value) in data.iter().enumerate() {
        if value == data[(i + offset) % data.len()] {
            result += value as u64
        }
    }
    result
}

fn parse(input: &str) -> Vec<u8> {
    input.trim().as_bytes().iter().map(|b| b - b'0').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(&parse("1122")), 3);
        assert_eq!(part1(&parse("1111")), 4);
        assert_eq!(part1(&parse("1234")), 0);
        assert_eq!(part1(&parse("91212129")), 9);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse("1212")), 6);
        assert_eq!(part2(&parse("1221")), 0);
        assert_eq!(part2(&parse("123425")), 4);
        assert_eq!(part2(&parse("123123")), 12);
        assert_eq!(part2(&parse("12131415")), 4);
    }
}
