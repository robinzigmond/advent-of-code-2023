use std::fs::File;
use std::io::prelude::*;


fn read_file() -> Vec<Vec<i32>> {
  let mut file = File::open("./input/input9.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut nums = vec![];

  for line in contents.lines() {
    let mut nums_in_line = vec![];
    for num in line.split(" ") {
      nums_in_line.push(num.parse().unwrap());
    }
    nums.push(nums_in_line);
  }

  nums
}

fn get_differences(nums: &Vec<i32>) -> Vec<i32> {
  let (first, rest) = nums.split_at(1);
  if rest.len() == 0 {
    panic!("can't get differences of a 1-element sequence!");
  }
  let mut previous = first[0];
  let mut diffs = vec![];
  for &num in rest {
    diffs.push(num - previous);
    previous = num;
  }
  diffs
}

fn get_next_number(nums: Vec<i32>) -> i32 {
  let mut sequence = nums;
  let mut answer = sequence[sequence.len() - 1];
  while !sequence.iter().all(|&n| n == 0) {
    sequence = get_differences(&sequence);
    answer += sequence[sequence.len() - 1];
  }
  answer
}

fn solve_part_1(nums: Vec<Vec<i32>>) -> i32 {
  nums.into_iter().map(get_next_number).sum()
}

pub fn part_1() -> i32 {
  let nums = read_file();
  solve_part_1(nums)
}

fn get_previous_number(nums: Vec<i32>) -> i32 {
  let mut sequence = nums;
  let mut all_previous = vec![];
  while !sequence.iter().all(|&n| n == 0) {
    all_previous.push(sequence[0]);
    sequence = get_differences(&sequence);
  }
  let mut answer = 0;
  all_previous.reverse();
  for num in all_previous {
    answer = num - answer;
  }
  answer
}

fn solve_part_2(nums: Vec<Vec<i32>>) -> i32 {
  nums.into_iter().map(get_previous_number).sum()
}

pub fn part_2() -> i32 {
  let nums = read_file();
  solve_part_2(nums)
}
