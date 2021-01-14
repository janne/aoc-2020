use regex::Regex;
use std::fmt::Debug;
use std::str::FromStr;

pub fn read_input<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let data = std::fs::read_to_string(filename).unwrap();
    let data: Vec<T> = data
        .lines()
        .map(|line| line.parse::<T>().unwrap())
        .collect();
    data
}

pub fn instances_of(sub: &str, text: &str) -> i32 {
    let re = Regex::new(sub).unwrap();
    re.captures_iter(text).count() as i32
}
