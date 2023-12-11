use std::fs::File;
use std::io::prelude::*;

// collect relevant info about puzzle - some of this is redundant but it's
// all at least important for part 1 and we can easily collect it all directly
// as we process the input, line-by-line

struct PuzzleData {
  empty_rows: Vec<usize>,
  empty_cols: Vec<usize>,
  galaxies: Vec<(usize, usize)>,
}

fn read_file() -> PuzzleData {
  let mut file = File::open("./input/input11.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut non_empty_cols = vec![];
  let mut empty_rows = vec![];
  let mut galaxies = vec![];
  let mut width = 0;
  for (row_index, line) in contents.lines().enumerate() {
    let mut is_empty = true;
    for (col_index, char) in line.chars().enumerate() {
      // this is a poor and lazy way to get the width, but it was easy and is good enough!
      width = col_index;
      if char == '#' {
        non_empty_cols.push(col_index);
        is_empty = false;
        galaxies.push((col_index, row_index));
      }
    }
    if is_empty {
      empty_rows.push(row_index);
    }
  }
  let empty_cols = (0..width).filter(|n| !non_empty_cols.contains(n)).collect();

  PuzzleData { empty_rows, empty_cols, galaxies }
}

// common utility, used for both parts 1 and 2 since they differ in only one "small" detail

fn get_total(data: &PuzzleData, expansion_factor: u64) -> u64 {
  let num_of_galaxies = data.galaxies.len();
  let mut answer = 0;
  for i in 0..num_of_galaxies {
    for j in (i+1)..num_of_galaxies {
      let (x0, y0) = data.galaxies[i];
      let (x1, y1) = data.galaxies[j];
      // note that these ranges include exactly one endpoint, which is correct - it doesn't matter which we take.
      // The endpoints obviously can't correspond to empty rows or columns, as they contain galaxies!
      let x_range = if x0 < x1 { x0..x1 } else { x1..x0 };
      let y_range = if y0 < y1 { y0..y1 } else { y1..y0 };
      let mut manhattan_distance = 0;
      for x in x_range {
        manhattan_distance += if data.empty_cols.contains(&x) { expansion_factor } else {1 };
      }
      for y in y_range {
        manhattan_distance += if data.empty_rows.contains(&y) { expansion_factor } else {1 };
      }
      answer += manhattan_distance;
    }
  }
  answer
}

fn solve_part_1(data: &PuzzleData) -> u64 {
  get_total(data, 2)
}

pub fn part_1() -> u64 {
  let data = read_file();
  solve_part_1(&data)
}

fn solve_part_2(data: &PuzzleData) -> u64 {
  get_total(data, 1000000)
}

pub fn part_2() -> u64 {
  let data = read_file();
  solve_part_2(&data)
}
