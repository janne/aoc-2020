use std::fs;

pub fn task_a() -> i32 {
    let text = fs::read_to_string("inputs/day4.txt").unwrap();
    let passports = text.split("\n\n");
    let passports: Vec<Vec<&str>> = passports
        .map(|pp| pp.split(|c| c == '\n' || c == ' ').collect::<Vec<&str>>())
        .collect();
    println!("First row: {:?}", passports[0]);
    return passports.len() as i32;
}
