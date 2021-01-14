use crate::utils;

pub fn task_a() -> i32 {
  let values: Vec<i32> = utils::read_input("inputs/day1.txt");
  for i in &values {
    for j in &values {
      if i + j == 2020 {
        return i * j;
      }
    }
  }
  panic!("No result found");
}

pub fn task_b() -> i32 {
  let values: Vec<i32> = utils::read_input("inputs/day1.txt");
  for i in &values {
    for j in &values {
      for k in &values {
        if i + j + k == 2020 {
          return i * j * k;
        }
      }
    }
  }
  panic!("No result found");
}
