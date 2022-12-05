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

struct Instruction {
    kind: InstructionKind,
    register: RegisterIndex,
    amount: i64,
    condition: Condition,
}

enum InstructionKind {
    Inc,
    Dec,
}

impl Instruction {
    fn parse<'a>(line: &'a str, mut registers: Registers<'a>) -> (Self, Registers<'a>) {
        use InstructionKind::*;
        let mut parts = line.trim().split(" if ");

        let mut tokens = parts.next().unwrap().split_whitespace();
        let register = registers.register_id(tokens.next().unwrap());
        let kind = match tokens.next().unwrap() {
            "inc" => Inc,
            "dec" => Dec,
            _ => unreachable!(),
        };
        let amount = tokens.next().unwrap().parse::<i64>().unwrap();
        let (condition, registers) = Condition::parse(parts.next().unwrap(), registers);

        (
            Instruction {
                kind,
                register,
                amount,
                condition,
            },
            registers,
        )
    }

    fn exec<'a, 'b>(&'a self, mut registers: Registers<'b>) -> Registers<'b> {
        use InstructionKind::*;
        if self.condition.check(&registers) {
            match self.kind {
                Inc => {
                    registers
                        .set_value(self.register, registers.value(self.register) + self.amount);
                }
                Dec => {
                    registers
                        .set_value(self.register, registers.value(self.register) - self.amount);
                }
            }
            registers
        } else {
            registers
        }
    }
}

struct Condition {
    kind: ConditionKind,
    register: RegisterIndex,
    amount: i64,
}

enum ConditionKind {
    Greater,
    Smaller,
    GreaterOrEqual,
    SmallerOrEqual,
    Equal,
    NotEqual,
}

impl Condition {
    fn parse<'a>(line: &'a str, mut registers: Registers<'a>) -> (Self, Registers<'a>) {
        use ConditionKind::*;

        let mut tokens = line.trim().split_whitespace();
        let register = registers.register_id(tokens.next().unwrap());
        let kind = match tokens.next().unwrap() {
            "<" => Smaller,
            ">" => Greater,
            "<=" => SmallerOrEqual,
            ">=" => GreaterOrEqual,
            "==" => Equal,
            "!=" => NotEqual,
            _ => unreachable!(),
        };
        let amount = tokens.next().unwrap().parse::<i64>().unwrap();
        (
            Condition {
                amount,
                kind,
                register,
            },
            registers,
        )
    }

    fn check(&self, registers: &Registers) -> bool {
        use ConditionKind::*;

        let reg = registers.value(self.register);
        match self.kind {
            Greater => reg > self.amount,
            Smaller => reg < self.amount,
            GreaterOrEqual => reg >= self.amount,
            SmallerOrEqual => reg <= self.amount,
            Equal => reg == self.amount,
            NotEqual => reg != self.amount,
        }
    }
}

fn solve(input: &str) -> (i64, i64) {
    let registers = input
        .trim()
        .lines()
        .fold(Registers::default(), |registers, line| {
            let (instruction, registers) = Instruction::parse(line, registers);
            instruction.exec(registers)
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
