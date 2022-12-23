use std::fs;

fn main() {
    let mut file_lines = Vec::new();
    let file = fs::read_to_string("input.txt").expect("omgz no file");
    for lines in file.lines() {
        file_lines.push(lines);
    }
    let mut bang: char = '0';
    let mut sum: i32 = 0;
    for line in file_lines {
        bang = '0';
        let (pock1, pock2) = line.split_at(line.len() / 2);
        for c in pock2.chars() {
            if pock1.contains(c) {
                if bang == c {
                    continue;
                } else {
                    bang = c;
                    if c.to_string().as_bytes()[0] < 96 {
                        let num = c.to_string().as_bytes()[0] as i32 - 38;
                        sum += num;
                    } else {
                        let num = c.to_string().as_bytes()[0] as i32 - 96;
                        sum += num;
                    }
                }
            }
        }
    }
    println!("{}", sum);
}
