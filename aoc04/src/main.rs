use std::fs;

fn main() {
    println!("{}", aoc4part1("input.txt"));
    println!("{}", aoc4part2("input.txt"));
}

fn aoc4part1(file: &str) -> usize {
    let input = fs::read_to_string(file).unwrap();
    let output = input
        .lines()
        .filter(|line| {
            let (elf1, elf2) = line.split_once(',').unwrap();
            let (minelf1, maxelf1) = elf1.split_once('-').unwrap();
            let (minelf2, maxelf2) = elf2.split_once('-').unwrap();
            let minelf1: u32 = minelf1.parse().unwrap();
            let maxelf1: u32 = maxelf1.parse().unwrap();
            let minelf2: u32 = minelf2.parse().unwrap();
            let maxelf2: u32 = maxelf2.parse().unwrap();
            (minelf1 <= minelf2 && maxelf1 >= maxelf2) || (minelf2 <= minelf1 && maxelf2 >= maxelf1)
        })
        .count();
    output
}

fn aoc4part2(file: &str) -> usize {
    let input = fs::read_to_string(file).unwrap();
    let output = input
        .lines()
        .filter(|line| {
            let (elf1, elf2) = line.split_once(',').unwrap();
            let (minelf1, maxelf1) = elf1.split_once('-').unwrap();
            let (minelf2, maxelf2) = elf2.split_once('-').unwrap();
            let minelf1: u32 = minelf1.parse().unwrap();
            let maxelf1: u32 = maxelf1.parse().unwrap();
            let minelf2: u32 = minelf2.parse().unwrap();
            let maxelf2: u32 = maxelf2.parse().unwrap();
            (minelf1 <= maxelf2) && (maxelf1 >= minelf2)
        })
        .count();
    output
}
