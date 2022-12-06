use std::fmt::{Display, Write};
use std::ops::BitXor;

const INPUT: &str = "97,167,54,178,2,11,209,174,119,248,254,0,255,1,64,190";
const ADD_TO_END: [usize; 5] = [17, 31, 73, 47, 23];

fn main() {
    let setup_time = std::time::Instant::now();
    let part1 = part1::<256>(INPUT);
    let part1_dur = setup_time.elapsed().as_micros();
    println!("Part1 : {} in {} µs", part1, part1_dur);

    let setup_time = std::time::Instant::now();
    let part2 = part2::<256>(INPUT);
    let part2_dur = setup_time.elapsed().as_micros();
    println!("Part2 : {} in {} µs", part2, part2_dur);
}

fn part1<const C: usize>(input: &str) -> u32 {
    ElvenHasher::<C>::new(input).solve::<1, TextDecoder, WeakHash>()
}

fn part2<const C: usize>(input: &str) -> String {
    ElvenHasher::<C>::new(input).solve::<64, AsciiDecoder, DenseHash>()
}

struct ElvenHasher<'a, const C: usize> {
    input: &'a str,
    skip_size: usize,
    position: usize,
    list: [u8; C],
}

impl<'a, const C: usize> ElvenHasher<'a, C> {
    fn new(input: &'a str) -> Self {
        let mut list = [0; C];
        for (i, item) in list.iter_mut().enumerate() {
            *item = i as u8;
        }
        Self {
            input,
            skip_size: 0,
            position: 0,
            list,
        }
    }

    fn sparse_hash<I: Iterator<Item = usize>>(&mut self, lengths: I) {
        for length in lengths {
            for i in 0..(length / 2) {
                self.list.swap(
                    (self.position + i) % C,
                    (self.position + length - i - 1) % C,
                );
            }
            self.position += length + self.skip_size;
            self.skip_size += 1;
        }
    }

    fn solve<const REPEATS: u8, Decoder, H>(&mut self) -> H::Output
    where
        Decoder: LengthDecoder,
        H: Hash,
    {
        for _ in 0..REPEATS {
            let lengths = Decoder::decode(self.input);
            self.sparse_hash(lengths);
        }
        H::hash(&self.list)
    }
}

trait Hash {
    type Output: Display;
    fn hash(list: &[u8]) -> Self::Output;
}

struct DenseHash;

impl Hash for DenseHash {
    type Output = String;
    fn hash(list: &[u8]) -> Self::Output {
        let mut result = String::with_capacity(16 * 2);
        list.chunks(16)
            .map(|block| block.into_iter().fold(0, |result, &c| result.bitxor(c)))
            .for_each(|c| write!(result, "{c:02x}").unwrap());
        result
    }
}

struct WeakHash;

impl Hash for WeakHash {
    type Output = u32;
    fn hash(list: &[u8]) -> Self::Output {
        list[0] as u32 * list[1] as u32
    }
}

trait LengthDecoder {
    fn decode<'a>(input: &'a str) -> Box<dyn Iterator<Item = usize> + 'a>;
}

struct TextDecoder;

impl LengthDecoder for TextDecoder {
    fn decode<'a>(input: &'a str) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(input.split(',').map(|s| s.parse().unwrap()))
    }
}

struct AsciiDecoder;

impl LengthDecoder for AsciiDecoder {
    fn decode<'a>(input: &'a str) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(
            input
                .as_bytes()
                .iter()
                .map(|&b| b as usize)
                .chain(ADD_TO_END.into_iter()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,1,5";

    #[test]
    fn test_parse_lengths() {
        assert_eq!(
            TextDecoder::decode(TEST_INPUT).collect::<Vec<usize>>(),
            [3, 4, 1, 5]
        );
        assert_eq!(
            AsciiDecoder::decode("1,2,3").collect::<Vec<usize>>(),
            [49, 44, 50, 44, 51, 17, 31, 73, 47, 23]
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1::<5>(TEST_INPUT), 12);
        assert_eq!(part1::<256>(INPUT), 8536);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::<256>(INPUT), "aff593797989d665349efe11bb4fd99b");
    }
}
