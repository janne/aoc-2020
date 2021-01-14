extern crate regex;

mod day1;
mod day2;
mod day3;
mod day4;
mod utils;

fn main() {
    println!("Day 1A = {}", day1::task_a());
    println!("Day 1B = {}", day1::task_b());
    println!("Day 2A = {}", day2::task_a());
    println!("Day 2B = {}", day2::task_b());
    println!("Day 3A = {}", day3::task_a());
    println!("Day 4A = {}", day4::task_a());
}
