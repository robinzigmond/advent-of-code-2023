use std::fs::File;
use std::io::prelude::*;

fn read_file() -> Vec<String> {
  let mut file = File::open("./input/input15.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  contents.split(",").map(|s| s.to_owned()).collect()
}

fn hash(instruction: &str) -> usize {
  let mut current = 0;

  for c in instruction.chars() {
    current += c as usize; // this cast to number apparently gets the ASCII value
    current *= 17;
    current %= 256;
  }

  current
}

fn solve_part_1(instructions: Vec<String>) -> usize {
  instructions.iter().map(|i| hash(i)).sum()
}

pub fn part_1() -> usize {
  let instructions = read_file();
  solve_part_1(instructions)
}

enum Instruction {
  Remove(String),
  Place(String, usize),
}

impl Instruction {
  fn from_str(str: &str) -> Self {
    let mut label = String::new();
    let mut found_equals = false;
    for c in str.chars() {
      match c {
        '=' => {
          found_equals = true;
        },
        '-' => {
          // this - will be the last character
          return Instruction::Remove(label);
        }
        _ => {
          if found_equals {
            // we know the focal length is only a single digit, so this must be the last character
            // and it must be a digit
            return Instruction::Place(label, c.to_string().parse().unwrap());
          } else {
            label.push(c);
          }
        }
      }
    }
    panic!("empty instruction - can't happen but this is to satisfy the compiler");
  }
}

// despite the hints in the puzzle, I think the best approach here is to store the data as an array of
// the 256 boxes, each containing a vector of lenses, which contains information on name and focal length.
// While this is less efficient than a hashmap for looking up by name, we're mostly looking in a specific
// box and re-ordering/replacing things within a box, and that's much more convenient when the data is stored
// the way it is below.

struct Lens {
  label: String,
  focal_length: usize,
}

struct Boxes {
  // inner vector will always have length exactly 256, but it proves easier to use than
  // a fixed-length array of that size!
  content: Vec<Vec<Lens>>,
}

impl Boxes {
  fn follow_step(&mut self, instruction: String) {
    let instruction_to_follow = Instruction::from_str(&instruction);
    match instruction_to_follow {
      Instruction::Remove(label) => {
        let box_index = hash(&label);

        let existing_lens = self
          .content[box_index]
          .iter()
          .enumerate()
          .find(|(_, lens)| lens.label == label);

        if let Some((index, _)) = existing_lens {
          self.content[box_index].remove(index);
        }
      },
      Instruction::Place(label, focal_length) => {
        let box_index = hash(&label);

        let existing_lens = self
          .content[box_index]
          .iter()
          .enumerate()
          .find(|(_, lens)| lens.label == label);

        let new_lens = Lens { label, focal_length };

        match existing_lens {
          Some((index, _)) => {
            self.content[box_index][index] = new_lens;
          },
          None => {
            self.content[box_index].push(new_lens);
          },
        }
      },
    }
  }

  fn focusing_power(&self) -> usize {
    self.content.iter().enumerate().map(
      |(box_index, lenses)|
        lenses.iter().enumerate().map(
          |(box_position, lens)| (box_index + 1) * (box_position + 1) * lens.focal_length 
        ).sum::<usize>()
    ).sum()
  }
}

fn solve_part_2(instructions: Vec<String>) -> usize {
  let mut content = vec![];
  for _ in 0..256 {
    content.push(vec![]);
  }
  let mut boxes = Boxes { content };
  for instruction in instructions {
    boxes.follow_step(instruction);
  }
  boxes.focusing_power()
}

pub fn part_2() -> usize {
  let instructions = read_file();
  solve_part_2(instructions)
}
