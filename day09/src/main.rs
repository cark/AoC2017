const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let (part1, part2) = solve(INPUT);
    let dur = setup_time.elapsed().as_micros();
    println!("Solve time: {} µs", dur);
    println!("Part1 : {}", part1);
    println!("Part1 : {}", part2);
}

fn solve(input: &str) -> (u32, u32) {
    StateMachine::new().run(input)
}

struct StateMachine {
    state: State,
    depth: u32,
    score: u32,
    garbage_count: u32,
}

enum State {
    Start,
    Group,
    Garbage,
    Escape,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::Start,
            depth: 0,
            score: 0,
            garbage_count: 0,
        }
    }

    fn enter_group(&mut self) {
        self.state = State::Group;
        self.depth += 1;
        self.score += self.depth;
    }

    fn run(&mut self, input: &str) -> (u32, u32) {
        use State::*;
        for &char in input.as_bytes() {
            if char::from(char).is_whitespace() {
                continue;
            };
            match &self.state {
                Start => {
                    if char == b'{' {
                        self.enter_group();
                    } else {
                        panic!();
                    }
                }
                Group => match char {
                    b'{' => self.enter_group(),
                    b'}' => self.depth -= 1,
                    b'<' => self.state = Garbage,
                    _ => {}
                },
                Garbage => match char {
                    b'>' => self.state = Group,
                    b'!' => self.state = Escape,
                    _ => self.garbage_count += 1,
                },
                Escape => self.state = Garbage,
            }
        }
        (self.score, self.garbage_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both() {
        assert_eq!(solve(INPUT), (16827, 7298));
    }
}
