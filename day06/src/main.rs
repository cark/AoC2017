use std::collections::HashSet;

const INPUT: &str = "4	1	15	12	0	9	9	5	5	8	7	3	14	5	12	3";

fn main() {
    let setup_time = std::time::Instant::now();
    let part1 = part1(INPUT);
    let part1_dur = setup_time.elapsed().as_micros();
    println!("Part1 : {} in {} µs", part1, part1_dur);

    let setup_time = std::time::Instant::now();
    let part2 = part2(INPUT);
    let part2_dur = setup_time.elapsed().as_micros();
    println!("Part2 : {} in {} µs", part2, part2_dur);
}

fn part1(input: &str) -> u64 {
    let mut banks = parse_banks(input);
    let mut seen = HashSet::new();
    iterate(&mut banks, &mut seen)
}

fn part2(input: &str) -> u64 {
    let mut banks = parse_banks(input);
    let mut seen = HashSet::new();
    iterate(&mut banks, &mut seen);
    iterate(&mut banks, &mut seen)
}

fn iterate(banks: &mut [u8], seen: &mut HashSet<u64>) -> u64 {
    seen.clear();
    let mut iteration_count = 0;
    while !seen.contains(&perfect_hash(&banks)) {
        seen.insert(perfect_hash(&banks));
        let distribute_from = highest_index(&banks);
        distribute(banks, distribute_from);
        iteration_count += 1;
    }
    iteration_count
}

fn perfect_hash(banks: &[u8]) -> u64 {
    let mut result = 0;
    for i in 0..banks.len() {
        result += (banks[i] as u64) << (i * 4);
    }
    result
}

fn highest_index(banks: &[u8]) -> usize {
    let mut highest = 0;
    let mut highest_index = 0;
    for (i, val) in banks.iter().enumerate() {
        if *val > highest {
            highest = *val;
            highest_index = i;
        }
    }
    highest_index
}

fn distribute(banks: &mut [u8], mut index: usize) {
    let len = banks.len();
    let current = banks.get_mut(index).unwrap();
    let mut remaining = *current;
    //let inc_each_by = remaining / len;
    *current = 0;
    index += 1;
    while remaining > 0 {
        banks[index % len] += 1;
        remaining -= 1;
        index += 1;
    }
}

fn parse_banks(input: &str) -> Vec<u8> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 2 7 0";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 5);
        assert_eq!(part1(INPUT), 6681);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
        assert_eq!(part2(INPUT), 2392);
    }
}
