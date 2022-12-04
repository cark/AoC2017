use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let tower = Tower::parse(INPUT);
    let parsing_dur = setup_time.elapsed().as_micros();
    println!("Parsing: {} µs", parsing_dur);

    let setup_time = std::time::Instant::now();
    let part1 = Tower::part1(&tower);
    let part1_dur = setup_time.elapsed().as_nanos();
    println!("Part1 : {} in {} ns", part1, part1_dur);

    let setup_time = std::time::Instant::now();
    let part2 = Tower::part2(&tower);
    let part2_dur = setup_time.elapsed().as_nanos();
    println!("Part2 : {} in {} ns", part2, part2_dur);
}

#[cfg(test)]
fn part1(input: &str) -> &str {
    Tower::part1(&Tower::parse(input))
}

#[cfg(test)]
fn part2(input: &str) -> u64 {
    Tower::part2(&Tower::parse(input))
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

    fn prog<'b>(&'b self, prog_id: usize) -> &'b Program<'a> {
        self.items.get(prog_id).unwrap()
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
                let child_id = result.id_by_name(s.split(',').next().unwrap());
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

    fn part2(this: &Tower) -> u64 {
        fn recurse(this: &Tower, prog_id: usize) -> Result<u64, u64> {
            let prog = this.prog(prog_id);
            let weights = prog
                .children
                .iter()
                .map(|&child_id| recurse(this, child_id))
                .collect::<Result<Vec<u64>, u64>>();
            match weights {
                Err(value) => Err(value),
                Ok(weights) => {
                    if weights.iter().all(|&x| x == weights[0]) {
                        Ok(prog.weight + weights.iter().sum::<u64>())
                    } else {
                        let biggest = weights.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
                        let biggest_weight = weights[biggest];
                        let other_weights = weights[(biggest + 1) % weights.len()];
                        let own_weight = this.prog(prog.children[biggest]).weight;
                        let return_value = own_weight - (biggest_weight - other_weights);
                        Err(return_value)
                    }
                }
            }
        }
        let Err(value) = recurse(this, this.root_index()) else { panic!("boooh") };
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "tknk");
        assert_eq!(part1(INPUT), "cqmvs");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 60);
        assert_eq!(part2(INPUT), 2310);
    }
}
