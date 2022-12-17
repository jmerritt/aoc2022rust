use std::fs;

fn main() {
    // get the stuff with the things
    let file = fs::read_to_string("input.txt")
        .expect("couldn't read file");

    // initialize things
    let mut sum: u32 = 0;
    let mut sums = Vec::<u32>::new();

    // sort lines and do the math
    for line in file.lines() {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum = sum + line.parse::<u32>().unwrap();
        }
    }

    // part one answer
    println!("the elf with the most cals is: {:#?}", sums.iter().max().unwrap());

    sums.sort();
    sums.reverse();

    // part two answer
    println!("the top three elves total: {:#?}", sums[..3].iter().sum::<u32>());
}
