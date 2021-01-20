use std::collections::HashMap;
use std::fs;

fn get_passports<'a>(text: &'a str) -> Vec<HashMap<&'a str, &'a str>> {
    let passports = text.split("\n\n");
    return passports
        .map(|pp| pp.split(|c| c == '\n' || c == ' ').collect())
        .map(|pp: Vec<&str>| {
            pp.iter().fold(HashMap::new(), |mut acc, pp| {
                let pair: Vec<&str> = pp.split(":").collect();
                acc.insert(pair[0], pair[1]);
                return acc;
            })
        })
        .collect();
}

pub fn task_a() -> i32 {
    let text = fs::read_to_string("inputs/day4.txt").unwrap();
    let passports = get_passports(&text);
    let count = passports
        .iter()
        .filter(|pp| {
            pp.contains_key("byr")
                && pp.contains_key("iyr")
                && pp.contains_key("eyr")
                && pp.contains_key("hgt")
                && pp.contains_key("hcl")
                && pp.contains_key("ecl")
                && pp.contains_key("pid")
        })
        .count();

    return count as i32;
}
