use std::fs::File;
use std::io::prelude::*;
use num::integer::lcm;

struct Node {
  name: String,
  left: String,
  right: String,
}

enum Direction {
  Left,
  Right,
}

struct Input {
  path: Vec<Direction>,
  network: Vec<Node>,
}

fn read_direction(c: &char) -> Direction {
  match c {
    'L' => Direction::Left,
    'R' => Direction::Right,
    _ => panic!("unexpected direction character: {}", c),
  }
}

fn read_node(s: &str) -> Node {
  // parts are always the same length so we can just get these from index positions
  let name = s.get(0..3).unwrap().to_owned();
  let left = s.get(7..10).unwrap().to_owned();
  let right = s.get(12..15).unwrap().to_owned();

  Node { name, left, right }
}

fn read_file() -> Input {
  let mut file = File::open("./input/input8.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let input_lines: Vec<&str> = contents.lines().collect();
  let path = input_lines[0].chars().map(|c| read_direction(&c)).collect();
  let network = input_lines.split_at(2).1.iter().map(|s| read_node(s)).collect();

  Input { path, network }
}

fn follow_path(network: &Vec<Node>, current: String, direction: &Direction) -> String {
  let current_node = network.into_iter().find(|n| n.name == current).unwrap();
  match direction {
    Direction::Left => current_node.left.clone(),
    Direction::Right => current_node.right.clone(),
  }
}

fn solve_part_1(input: &Input) -> u64 {
  let mut steps_taken = 0;
  let mut current_index = 0;
  let mut current_location = "AAA".to_owned();

  while current_location != "ZZZ" {
    let current_direction = &input.path[current_index];
    current_location = follow_path(&input.network, current_location, current_direction);
    current_index = (current_index + 1) % input.path.len();
    steps_taken += 1;
  }

  steps_taken
}

pub fn part_1() -> u64 {
  let network = read_file();
  solve_part_1(&network)
}

// This was run to reveal important information about when each of the parallel paths hits an end point:
#[allow(dead_code)]
pub fn get_info() {
  let input = read_file();
  let start_locations: Vec<String> = input.network.iter().map(|node| node.name.clone()).filter(|name| name.ends_with("A")).collect();
  let path_length = input.path.len();

  for location in start_locations {
    println!("starting at {}", location);
    let mut steps_taken = 0;
    let mut current_index = 0;
    let mut current_location = location;
    let mut zs_found = 0;
  
    while zs_found < 100 {
      let current_direction = &input.path[current_index];
      current_location = follow_path(&input.network, current_location, current_direction);
      current_index = (current_index + 1) % input.path.len();
      steps_taken += 1;
      if current_location.ends_with("Z") {
        println!("got to {} after {} steps - path length {}", current_location, steps_taken, path_length);
        zs_found += 1;
      }
    }
  }
}

// What running the above reveals is that the data has been specifically set up - because there is no way any of the below
// would happen by coincidence - so that some very nice properties hold.
// Each starting point goes to only a single __Z endpoint - never hitting any other possible endpoint. Further, these endpoints
// are hit for the first time after a whole number of cycles through the complete path (never midway through it), and after the
// same number of cycles (and at no point before) the same endpoint is hit again. The end result is that the number of steps
// after which a particular path is at an endpoint are all the integer multiples of a base number. This being so, the answer
// is simply the lowest common multiples of those "base numbers" for each starting point.
// The above needs to be know before the following simple solution will make sense:
fn solve_part_2(input: &Input) -> u64 {
  let start_locations: Vec<String> = input.network.iter().map(|node| node.name.clone()).filter(|name| name.ends_with("A")).collect();
  let mut all_base_numbers = vec![];

  for location in start_locations {
    let mut steps_taken = 0;
    let mut current_index = 0;
    let mut current_location = location;
  
    while !current_location.ends_with("Z") {
      let current_direction = &input.path[current_index];
      current_location = follow_path(&input.network, current_location, current_direction);
      current_index = (current_index + 1) % input.path.len();
      steps_taken += 1;
    }

    all_base_numbers.push(steps_taken);
  }

  // compute lcm
  let mut answer = 1;
  for number in all_base_numbers {
    answer = lcm(answer, number);
  }
  answer
}

pub fn part_2() -> u64 {
  let network = read_file();
  solve_part_2(&network)
}
