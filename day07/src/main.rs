use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let part1 = part1(INPUT);
    let part1_dur = setup_time.elapsed().as_micros();
    println!("Part1 : {} in {} µs", part1, part1_dur);
}

fn part1(input: &str) -> &str {
    Tower::part1(&Tower::parse(input))
}

struct Program<'a> {
    name: &'a str,
    weight: u64,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Program<'_> {
    fn set_parent(&mut self, parent_id: usize) {
        self.parent = Some(parent_id);
    }
    fn add_child(&mut self, child_id: usize) {
        self.children.push(child_id);
    }
}

#[derive(Default)]
struct Tower<'a> {
    items: Vec<Program<'a>>,
    name_to_index: HashMap<&'a str, usize>,
}

impl<'a> Tower<'a> {
    fn id_by_name<'b>(&'b mut self, name: &'a str) -> usize {
        if let Some(&id) = self.name_to_index.get(name) {
            id
        } else {
            let result = self.items.len();
            self.name_to_index.insert(name, result);
            self.items.push(Program {
                name,
                parent: None,
                weight: 0,
                children: vec![],
            });
            result
        }
    }

    fn prog_mut<'b>(&'b mut self, prog_id: usize) -> &'b mut Program<'a> {
        self.items.get_mut(prog_id).unwrap()
    }

    fn parse(input: &'a str) -> Tower<'a> {
        let mut result = Tower {
            items: vec![],
            name_to_index: HashMap::default(),
        };
        let mut lines = input.trim().lines();
        while let Some(line) = lines.next() {
            let mut parts = line.split_whitespace();
            let prog_id = result.id_by_name(parts.next().unwrap());
            let prog = result.prog_mut(prog_id);
            let weight = parts.next().unwrap();
            prog.weight = weight[1..weight.len() - 1].parse().unwrap();
            let mut parts = parts.skip(1);
            while let Some(s) = parts.next() {
                let child_id = result.id_by_name(&s[..s.len() - 1]);
                result.prog_mut(child_id).set_parent(prog_id);
                result.prog_mut(prog_id).add_child(child_id)
            }
        }
        result
    }

    fn part1<'b>(this: &Tower<'b>) -> &'b str {
        this.items[this.root_index()].name
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
        assert_eq!(part1(TEST_INPUT), "tknk");
        assert_eq!(part1(INPUT), "cqmvs");
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(TEST_INPUT), 60);
        // assert_eq!(part2(INPUT), 861);
    }
}
