use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_file() -> Vec<String> {
  let mut file = File::open("./input/input1.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents.lines().map(|n| n.to_string()).collect()
}

fn get_digit(line: &str, reverse: bool) -> i32 {
  let iterator = if reverse { line.chars().rev().collect::<Vec<char>>() } else { line.chars().collect() };
  for char in iterator {
    match char.to_string().parse() {
      Ok(n) => return n,
      Err(_) => (),
    }
  }
  panic!("no digit found in string!");
}

fn read_number(line: &String) -> i32 {
  let first_digit = get_digit(line, false);
  let last_digit = get_digit(line, true);
  10 * first_digit + last_digit
}

fn solve_part_1(v: Vec<String>) -> i32 {
  v.iter().map(read_number).sum()
}

pub fn part_1() -> i32 {
    let nums = read_file();
    solve_part_1(nums)
}

fn get_digit_2(line: &str, reverse: bool) -> i32 {
  let mut number_names = HashMap::new();
  number_names.insert("one", 1);
  number_names.insert("two", 2);
  number_names.insert("three", 3);
  number_names.insert("four", 4);
  number_names.insert("five", 5);
  number_names.insert("six", 6);
  number_names.insert("seven", 7);
  number_names.insert("eight", 8);
  number_names.insert("nine", 9);
  let iterator = if reverse { line.chars().rev().collect::<Vec<char>>() } else { line.chars().collect() };
  let mut index_limit = line.len();
  let mut found_digit = None;
  for (name, digit) in number_names.into_iter() {
    let string_to_find = if reverse { name.chars().rev().collect() } else { name.to_string() };
    match iterator.iter().collect::<String>().find(&string_to_find) {
      Some(j) => {
        if j < index_limit {
          index_limit = j;
          found_digit = Some(digit);
        }
      },
      None => {},
    }
  }
  for char in iterator.iter().take(index_limit) {
    match char.to_string().parse() {
      Ok(n) => return n,
      Err(_) => (),
    }
  }
  found_digit.unwrap()
}

fn read_number_2(line: &String) -> i32 {
  let first_digit = get_digit_2(line, false);
  let last_digit = get_digit_2(line, true);
  10 * first_digit + last_digit
}

fn solve_part_2(v: Vec<String>) -> i32 {
  v.iter().map(read_number_2).sum()
}

pub fn part_2() -> i32 {
    let nums = read_file();
    solve_part_2(nums)
}
