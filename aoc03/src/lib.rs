use std::fs;
use itertools::Itertools;

pub fn day3part1(input: &str) -> u32 {
    let input = fs::read_to_string(input).unwrap();
    input
        .lines()
        .filter_map(|line| {
            let line = line.as_bytes();
            let (pock_l, pock_r) = line.split_at(line.len() / 2);
            pock_l
                .iter()
                .find(|item| pock_r.contains(item))
                .map(|item| match item {
                    b'a'..=b'z' => (item - b'a') + 1,
                    _ => (item - b'A') + 1 + 26,
                } as u32)
        })
        .sum()
}

pub fn day3part2(input: &str) -> u32 {
    let input = fs::read_to_string(input).unwrap();
    input
        .lines()
        .map(|line| line.as_bytes())
        .tuples()
        .filter_map(|(bag1, bag2, bag3)| {
            bag1
                .iter()
                .find(|item| bag2.contains(item) && bag3.contains(item))
                .map(|item| match item {
                    b'a'..=b'z' => (item - b'a') + 1,
                    _ => (item - b'A') + 1 + 26,
                } as u32)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trypart1() {
        assert_eq!(day3part1("test.txt"), 36);
    }

    #[test]
    fn trypart2() {
        assert_eq!(day3part2("test.txt"), 4);
    }
}
