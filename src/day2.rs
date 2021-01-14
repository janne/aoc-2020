use crate::utils;
use regex::Regex;
use std::fs;

pub fn task_a() -> i32 {
    let text = fs::read_to_string("inputs/day2.txt").unwrap();
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    re.captures_iter(&text)
        .filter(|cap| {
            let min: i32 = cap[1].parse().unwrap();
            let max: i32 = cap[2].parse().unwrap();
            let inst = utils::instances_of(&cap[3], &cap[4]);
            return inst >= min && inst <= max;
        })
        .count() as i32
}

pub fn task_b() -> i32 {
    let text = fs::read_to_string("inputs/day2.txt").unwrap();
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    re.captures_iter(&text)
        .filter(|cap| {
            let chr: char = cap[3].chars().nth(0).unwrap();
            let mut line = cap[4].chars();
            let i: usize = cap[1].parse().unwrap();
            let j: usize = cap[2].parse().unwrap();
            let a: bool = line.nth(i - 1).unwrap() == chr;
            let b: bool = line.nth(j - i - 1).unwrap() == chr;
            return (a && !b) || (!a && b);
        })
        .count() as i32
}
