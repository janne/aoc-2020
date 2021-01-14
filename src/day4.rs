use std::fs;

pub fn task_a() -> i32 {
    let text = fs::read_to_string("inputs/day4.txt").unwrap();
    return text.len() as i32;
}
