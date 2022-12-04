use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let part1 = part1(INPUT);
    let part1_dur = setup_time.elapsed().as_micros();
    println!("Part1 : {} in {} µs", part1, part1_dur);
}

fn part1(input: &'static str) -> &'static str {
    Tower::parse(input).root_name()
}

struct Program {
    name: &'static str,
    weight: u64,
    parent: Option<usize>,
}

impl Program {
    fn set_parent(&mut self, parent_id: usize) {
        self.parent = Some(parent_id);
    }
}

#[derive(Default)]
struct Tower {
    items: Vec<Program>,
    name_to_index: HashMap<&'static str, usize>,
}

impl Tower {
    fn id_by_name(&mut self, name: &'static str) -> usize {
        if let Some(&id) = self.name_to_index.get(name) {
            id
        } else {
            let result = self.items.len();
            self.name_to_index.insert(name, result);
            self.items.push(Program {
                name,
                parent: None,
                weight: 0,
            });
            result
        }
    }

    fn prog_mut(&mut self, prog_id: usize) -> &mut Program {
        self.items.get_mut(prog_id).unwrap()
    }

    fn parse(input: &'static str) -> Tower {
        let mut result = Self::default();
        input.trim().lines().for_each(|line| {
            let mut parts = line.split_whitespace();
            let prog_id = result.id_by_name(parts.next().unwrap());
            let prog = result.prog_mut(prog_id);
            let weight = parts.next().unwrap();
            prog.weight = weight[1..weight.len() - 1].parse().unwrap();
            let mut parts = parts.skip(1);
            while let Some(s) = parts.next() {
                let id = result.id_by_name(&s[..s.len() - 1]);
                result.prog_mut(id).set_parent(prog_id);
            }
        });
        result
    }

    fn root_name(&self) -> &'static str {
        self.items[self.root_index()].name
    }

    fn root_index(&self) -> usize {
        let mut current = 0;
        while let Some(parent_id) = self.items[current].parent {
            current = parent_id
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_parse() {
        assert_eq!(Tower::parse(TEST_INPUT).root_name(), "tknk");
    }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(TEST_INPUT), 2);
    //     assert_eq!(part1(INPUT), 441);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 4);
    //     assert_eq!(part2(INPUT), 861);
    // }
}
