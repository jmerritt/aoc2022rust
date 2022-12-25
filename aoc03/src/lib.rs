use std::fs;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trypart1() {
        assert_eq!(day3part1("test.txt"), 3);
    }
}
