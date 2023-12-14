use std::fs::File;
use std::io::prelude::*;
use std::cmp;

#[derive(PartialEq, Clone, Copy)]
enum Space {
  Ash,
  Rock,
}

struct Pattern {
  grid: Vec<Vec<Space>>,
}

impl Pattern {
  // transposes the grid - ie flips about the NW-SE diagonal
  fn transpose(&self) -> Self {
    let mut new_grid = vec![];
    for (row_no, row) in self.grid.iter().enumerate() {
      for (col_no, &space) in row.iter().enumerate() {
        if row_no == 0 {
          new_grid.push(vec![]);
        }
        new_grid[col_no].push(space);
      }
    }
    Pattern { grid: new_grid }
  }
}

fn read_line(line: &str) -> Vec<Space> {
  line.chars().map(|c| match c {
    '.' => Space::Ash,
    '#' => Space::Rock,
    _ => panic!("unexpected character in input: {}", c),
  }).collect()
}

fn read_file() -> Vec<Pattern> {
  let mut file = File::open("./input/input13.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut patterns = vec![];
  let mut current_grid = vec![];

  for line in contents.lines() {
    if line.is_empty() {
      patterns.push(Pattern { grid: current_grid });
      current_grid = vec![];
    } else {
      current_grid.push(read_line(line));
    }
  }
  patterns.push(Pattern { grid: current_grid });

  patterns
}

fn get_mirror_row_index(pattern: &Pattern) -> Option<usize> {
  let Pattern { grid } = pattern;
  let grid_height = grid.len();
  for possible_mirror in 1..grid_height {
    let mut is_mirror = true;
    let space_above = possible_mirror;
    let space_below = grid_height - space_above;
    for i in 1..=(cmp::min(space_above, space_below)) {
      let row_above = &grid[possible_mirror - i];
      let row_below = &grid[possible_mirror + i - 1];
      if row_above != row_below {
        is_mirror = false;
        break;
      }
    }
    if is_mirror {
      return Some(possible_mirror);
    }
  }
  None
}

fn get_mirror_column_index(pattern: &Pattern) -> Option<usize> {
  let transposed = pattern.transpose();
  get_mirror_row_index(&transposed)
}

fn get_reflection_score(pattern: &Pattern) -> Option<usize> {
  match get_mirror_row_index(pattern) {
    Some(index) => Some(100 * index),
    None => get_mirror_column_index(pattern),
  }
}

fn solve_part_1(patterns: &Vec<Pattern>) -> usize {
  patterns.iter().map(|pattern| get_reflection_score(pattern).expect("no horizontal OR vertical reflection!")).sum()
}

pub fn part_1() -> usize {
  let patterns = read_file();
  solve_part_1(&patterns)
}

// we repeat most of the functionality of part 1, to now work assuming there is a "smudge".
// The below function looks a little complex but it's basically the same as the "smudge-less" version,
// except that:
// 1) we compute where the mirror line was before (if there is one) to ensure that we don't return
// that as the answer again (as the puzzle tells us that it's at least possible for fixing the smudge
// to leave this as a line of symmetry, but that it will definitely introduce a new one)
// 2) rather than simply giving up when we find any difference, we scan every cell and give up as soon
// as we have found a second difference.

fn get_mirror_row_index_with_smudge(pattern: &Pattern) -> Option<usize> {
  let previous_row_mirror_index = get_mirror_row_index(pattern);
  let Pattern { grid } = pattern;
  let grid_height = grid.len();
  for possible_mirror in 1..grid_height {
    if previous_row_mirror_index == Some(possible_mirror) {
      continue;
    }
    let mut is_mirror = true;
    let space_above = possible_mirror;
    let space_below = grid_height - space_above;
    'test_rows: for i in 1..=(cmp::min(space_above, space_below)) {
      let row_above = &grid[possible_mirror - i];
      let row_below = &grid[possible_mirror + i - 1];
      let mut has_difference = false;
      for j in 0..row_above.len() {
        if row_above[j] != row_below[j] {
          if has_difference {
            // there's more than one difference in the row, so there's no possibility
            // for this to be the correct horizontal mirror line
            is_mirror = false;
            break 'test_rows;
          }
          has_difference = true;
        }
      }
    }
    if is_mirror {
      return Some(possible_mirror);
    }
  }
  None
}

fn get_mirror_column_index_with_smudge(pattern: &Pattern) -> Option<usize> {
  let transposed = pattern.transpose();
  get_mirror_row_index_with_smudge(&transposed)
}

fn get_reflection_score_with_smudge(pattern: &Pattern) -> Option<usize> {
  match get_mirror_row_index_with_smudge(pattern) {
    Some(index) => Some(100 * index),
    None => get_mirror_column_index_with_smudge(pattern),
  }
}

fn solve_part_2(patterns: &Vec<Pattern>) -> usize {
  patterns.iter().map(|pattern| get_reflection_score_with_smudge(pattern).expect("no horizontal OR vertical reflection!")).sum()
}

pub fn part_2() -> usize {
  let patterns = read_file();
  solve_part_2(&patterns)
}
