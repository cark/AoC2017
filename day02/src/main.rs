const INPUT: &str = include_str!("input.txt");

fn main() {
    let data = parse(INPUT);
    println!("Part1 : {}", part1(&data));
    println!("Part2 : {}", part2(&data));
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|v| {
            let (min, max) = v.iter().fold((i32::MAX, i32::MIN), |(min, max), &val| {
                (min.min(val), max.max(val))
            });
            max - min
        })
        .sum()
}

fn find_divisibles(v: &Vec<i32>) -> (i32, i32) {
    for i in 0..v.len() {
        for j in 0..v.len() {
            if i == j {
                continue;
            }
            if (v[i] % v[j]) == 0 {
                return (v[i], v[j]);
            }
        }
    }
    unreachable!();
}

fn part2(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|v| {
            let (a, b) = find_divisibles(v);
            a / b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(TEST_INPUT)), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT2)), 9);
    }
}
