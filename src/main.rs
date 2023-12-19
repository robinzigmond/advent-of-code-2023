mod solutions;

use crate::solutions::*;
use std::thread;

// need a larger stack for day 16 puzzle using recursive approach!
// Found how to do it from https://www.reddit.com/r/rust/comments/872fc4/how_to_increase_the_stack_size/
const STACK_SIZE: usize = 4 * 1024 * 1024;

fn run() {
  println!("The answer to day 1, part 1 is {}", day1::part_1());
  println!("The answer to day 1, part 2 is {}", day1::part_2());
  println!("The answer to day 2, part 1 is {}", day2::part_1());
  println!("The answer to day 2, part 2 is {}", day2::part_2());
  println!("The answer to day 3, part 1 is {}", day3::part_1());
  println!("The answer to day 3, part 2 is {}", day3::part_2());
  println!("The answer to day 4, part 1 is {}", day4::part_1());
  println!("The answer to day 4, part 2 is {}", day4::part_2());
  println!("The answer to day 5, part 1 is {}", day5::part_1());
  println!("The answer to day 5, part 2 is {}", day5::part_2());
  println!("The answer to day 6, part 1 is {}", day6::part_1());
  println!("The answer to day 6, part 2 is {}", day6::part_2());
  println!("The answer to day 7, part 1 is {}", day7::part_1());
  println!("The answer to day 7, part 2 is {}", day7::part_2());
  println!("The answer to day 8, part 1 is {}", day8::part_1());
  // day8::get_info();
  println!("The answer to day 8, part 2 is {}", day8::part_2());
  println!("The answer to day 9, part 1 is {}", day9::part_1());
  println!("The answer to day 9, part 2 is {}", day9::part_2());
  println!("The answer to day 10, part 1 is {}", day10::part_1());
  println!("The answer to day 10, part 2 is {}", day10::part_2());
  println!("The answer to day 11, part 1 is {}", day11::part_1());
  println!("The answer to day 11, part 2 is {}", day11::part_2());
  println!("The answer to day 12, part 1 is {}", day12::part_1());
  println!("The answer to day 12, part 2 is {}", day12::part_2());
  println!("The answer to day 13, part 1 is {}", day13::part_1());
  println!("The answer to day 13, part 2 is {}", day13::part_2());
  println!("The answer to day 14, part 1 is {}", day14::part_1());
  println!("The answer to day 14, part 2 is {}", day14::part_2());
  println!("The answer to day 15, part 1 is {}", day15::part_1());
  println!("The answer to day 15, part 2 is {}", day15::part_2());
  println!("The answer to day 16, part 1 is {}", day16::part_1());
  println!("The answer to day 16, part 2 is {}", day16::part_2());
  println!("The answer to day 17, part 1 is {}", day17::part_1());
  println!("The answer to day 17, part 2 is {}", day17::part_2());
  println!("The answer to day 18, part 1 is {}", day18::part_1());
  println!("The answer to day 18, part 2 is {}", day18::part_2());
  println!("The answer to day 19, part 1 is {}", day19::part_1());
  println!("The answer to day 19, part 2 is {}", day19::part_2());
}

fn main() {
  // Spawn thread with explicit stack size
  let child = thread::Builder::new()
      .stack_size(STACK_SIZE)
      .spawn(run)
      .unwrap();

  // Wait for thread to join
  child.join().unwrap();
}
