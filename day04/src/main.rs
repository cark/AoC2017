use std::convert::identity;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let part1 = solve(INPUT, identity);
    let part1_dur = setup_time.elapsed().as_micros();
    println!("Part1 : {} in {} µs", part1, part1_dur);

    let setup_time = std::time::Instant::now();
    let part2 = solve(INPUT, sort_chars);
    let part2_dur = setup_time.elapsed().as_micros();
    println!("Part2 : {} in {} µs", part2, part2_dur);
}

fn solve<'a, T: PartialEq>(input: &'a str, map_word: impl Fn(&'a str) -> T) -> i32 {
    input
        .trim()
        .lines()
        .filter(|&line| has_no_duplicates(word_vec(line, &map_word)))
        .count() as i32
}

fn has_no_duplicates<T: PartialEq>(words: Vec<T>) -> bool {
    for i in 0..words.len() {
        for j in 0..words.len() {
            if i != j && words[i] == words[j] {
                return false;
            }
        }
    }
    return true;
}

fn word_vec<'a, T>(line: &'a str, map: &impl Fn(&'a str) -> T) -> Vec<T> {
    line.split_whitespace().map(map).collect::<Vec<T>>()
}

fn sort_chars(word: &str) -> Vec<u8> {
    let mut result = word.as_bytes().to_vec();
    result.sort_unstable();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve(INPUT, identity), 466);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(INPUT, sort_chars), 251);
    }
}
