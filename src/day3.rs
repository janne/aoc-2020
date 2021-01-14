use std::fs;

pub fn task_a() -> i32 {
    let text = fs::read_to_string("inputs/day3.txt").unwrap();
    let mut count = 0;
    for (col, line) in text.lines().enumerate() {
        let chr = line.chars().nth((col * 3) % 31).unwrap();
        if chr == '#' {
            count += 1;
        }
    }
    return count;
}
