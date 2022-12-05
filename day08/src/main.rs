use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let solution = solve(INPUT);
    let solution_dur = setup_time.elapsed().as_micros();
    println!("Solution in : {} µs", solution_dur);
    println!("Part1 : {}", solution.0);
    println!("Part2 : {}", solution.1);
}

struct Registers<'a> {
    items: Vec<i64>,
    names: HashMap<&'a str, usize>,
    highest: i64,
}

impl Default for Registers<'_> {
    fn default() -> Self {
        Self {
            highest: i64::MIN,
            items: Default::default(),
            names: Default::default(),
        }
    }
}

#[derive(Clone, Copy)]
struct RegisterIndex(usize);

impl<'a> Registers<'a> {
    fn register_id(&mut self, name: &'a str) -> RegisterIndex {
        if let Some(&i) = self.names.get(name) {
            RegisterIndex(i)
        } else {
            let result = RegisterIndex(self.items.len());
            self.names.insert(name, result.0);
            self.items.push(0);
            result
        }
    }

    fn value(&self, index: RegisterIndex) -> i64 {
        self.items[index.0]
    }

    fn set_value(&mut self, index: RegisterIndex, value: i64) {
        self.highest = self.highest.max(value);
        self.items[index.0] = value;
    }
}

fn parse_and_exec_instruction<'a>(line: &'a str, mut registers: Registers<'a>) -> Registers<'a> {
    let mut parts = line.trim().split(" if ");

    let mut tokens = parts.next().unwrap().split_whitespace();
    let register = registers.register_id(tokens.next().unwrap());
    let op = tokens.next().unwrap();
    let amount = tokens.next().unwrap().parse::<i64>().unwrap();
    let (result, mut registers) = parse_and_check_condition(parts.next().unwrap(), registers);
    if result {
        match op {
            "inc" => registers.set_value(register, registers.value(register) + amount),
            "dec" => registers.set_value(register, registers.value(register) - amount),
            _ => unreachable!(),
        };
    }
    registers
}

fn parse_and_check_condition<'a>(
    line: &'a str,
    mut registers: Registers<'a>,
) -> (bool, Registers<'a>) {
    let mut tokens = line.trim().split_whitespace();
    let register_id = registers.register_id(tokens.next().unwrap());
    let register = registers.value(register_id);
    let op = tokens.next().unwrap();
    let amount = tokens.next().unwrap().parse::<i64>().unwrap();
    (
        match op {
            "<" => register < amount,
            ">" => register > amount,
            "<=" => register <= amount,
            ">=" => register >= amount,
            "==" => register == amount,
            "!=" => register != amount,
            _ => unreachable!(),
        },
        registers,
    )
}

fn solve(input: &str) -> (i64, i64) {
    let registers = input
        .trim()
        .lines()
        .fold(Registers::default(), |registers, line| {
            parse_and_exec_instruction(line, registers)
        });
    (
        registers.items.into_iter().max().unwrap(),
        registers.highest,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_both() {
        assert_eq!(solve(TEST_INPUT), (1, 10));
        assert_eq!(solve(INPUT), (4567, 5636));
    }
}
