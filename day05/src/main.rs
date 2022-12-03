const INPUT: &str = include_str!("input.txt");

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

fn part1(input: &str) -> i32 {
    run(load_program(input), |i| i + 1)
}

fn part2(input: &str) -> i32 {
    run(load_program(input), |i| if i >= 3 { i - 1 } else { i + 1 })
}

fn run(mut program: Vec<i32>, update_instruction: impl Fn(i32) -> i32) -> i32 {
    let mut ip: i32 = 0;
    let mut instruction_count = 0;
    while let Some(instruction) = program.get_mut(ip as usize) {
        ip += *instruction;
        *instruction = update_instruction(*instruction);
        instruction_count += 1;
    }
    instruction_count
}

fn load_program(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 5);
        assert_eq!(part1(INPUT), 388611);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 10);
        assert_eq!(part2(INPUT), 27763113);
    }
}
