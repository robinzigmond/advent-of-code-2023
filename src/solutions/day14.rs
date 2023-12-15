use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy)]
enum Rock {
  Cube,
  Round,
  Empty,
}

struct Rocks {
  grid: Vec<Vec<Rock>>,
}

impl Rocks {
  fn roll_north(&mut self, row: usize, col: usize) {
    let mut finishing_row = row;
    while finishing_row > 0 {
      let next_row = finishing_row - 1;
      let occupier = self.grid[next_row][col];
      if let Rock::Empty = occupier {
        finishing_row -= 1;
      } else {
        break;
      }
    }
    self.grid[row][col] = Rock::Empty;
    self.grid[finishing_row][col] = Rock::Round;
  }

  fn roll_west(&mut self, row: usize, col: usize) {
    let mut finishing_col = col;
    while finishing_col > 0 {
      let next_col = finishing_col - 1;
      let occupier = self.grid[row][next_col];
      if let Rock::Empty = occupier {
        finishing_col -= 1;
      } else {
        break;
      }
    }
    self.grid[row][col] = Rock::Empty;
    self.grid[row][finishing_col] = Rock::Round;
  }

  fn roll_south(&mut self, row: usize, col: usize) {
    let mut finishing_row = row;
    while finishing_row < self.grid.len() - 1 {
      let next_row = finishing_row + 1;
      let occupier = self.grid[next_row][col];
      if let Rock::Empty = occupier {
        finishing_row += 1;
      } else {
        break;
      }
    }
    self.grid[row][col] = Rock::Empty;
    self.grid[finishing_row][col] = Rock::Round;
  }

  fn roll_east(&mut self, row: usize, col: usize) {
    let mut finishing_col = col;
    while finishing_col < self.grid[0].len() - 1 {
      let next_col = finishing_col + 1;
      let occupier = self.grid[row][next_col];
      if let Rock::Empty = occupier {
        finishing_col += 1;
      } else {
        break;
      }
    }
    self.grid[row][col] = Rock::Empty;
    self.grid[row][finishing_col] = Rock::Round;
  }

  fn roll_all_north(&mut self) {
    // note that by traversing in this "natural" order we avoid encountering again any rock
    // that we've already rolled!
    for (row_index, row) in self.grid.clone().iter().enumerate() {
      for (col, rock) in row.iter().enumerate() {
        if let Rock::Round = rock {
          self.roll_north(row_index, col);
        }
      }
    }
  }

  fn roll_all_west(&mut self) {
    // again the natural order works here
    for (row_index, row) in self.grid.clone().iter().enumerate() {
      for (col, rock) in row.iter().enumerate() {
        if let Rock::Round = rock {
          self.roll_west(row_index, col);
        }
      }
    }
  }

  fn roll_all_south(&mut self) {
    // this time we have to traverse the rows in reverse order
    let mut row_index = self.grid.len() - 1;
    loop {
      let row = &self.grid.clone()[row_index];
      for (col, rock) in row.iter().enumerate() {
        if let Rock::Round = rock {
          self.roll_south(row_index, col);
        }
      }
      if row_index == 0 {
        break;
      }
      row_index -= 1;
    }
  }

  fn roll_all_east(&mut self) {
    // this time it's the columns we have to iterate in reverse order
    for (row_index, row) in self.grid.clone().iter().enumerate() {
      let mut col = row.len() - 1;
      loop {
        let rock = row[col];
        if let Rock::Round = rock {
          self.roll_east(row_index, col);
        }
        if col == 0 {
          break;
        }
        col -= 1;
      }
    }
  }

  fn complete_cyle(&mut self) {
    self.roll_all_north();
    self.roll_all_west();
    self.roll_all_south();
    self.roll_all_east();
  }

  fn total_load_north(&self) -> usize {
    let mut total = 0;
    for (row_index, row) in self.grid.iter().enumerate() {
      for rock in row {
        if let Rock::Round = rock {
          total += self.grid.len() - row_index;
        }
      }
    }
    total
  }

  fn get_string_representation(&self) -> String {
    let mut str = String::new();
    for (row_index, row) in self.grid.iter().enumerate() {
      for (col_index, rock) in row.iter().enumerate() {
        if let Rock::Round = rock {
          str.push_str(&format!("{}{}", row_index, col_index));
        }
      }
    }
    str
  }
}

fn read_file() -> Rocks {
  let mut file = File::open("./input/input14.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut grid = vec![];
  for line in contents.lines() {
    let mut row = vec![];
    for c in line.chars() {
      let rock = match c {
        'O' => Rock::Round,
        '#' => Rock::Cube,
        '.' => Rock::Empty,
        _ => panic!("unexpected rock character: {}", c),
      };
      row.push(rock);
    }
    grid.push(row);
  }

  Rocks { grid }
}

fn solve_part_1(rocks: &mut Rocks) -> usize {
  rocks.roll_all_north();
  rocks.total_load_north()
}

pub fn part_1() -> usize {
  let mut rocks = read_file();
  solve_part_1(&mut rocks)
}

// As often with AoC problems, when presented with an impossible huge number of iterations to make,
// the solution is that a cycle occurs somewhere, allowing us to compute the result in a tiny
// fraction of the total we need. This works with the example so presumably will (with a longer, later cycle
// I assume) with the real data.
fn solve_part_2(rocks: &mut Rocks) -> usize {
  let mut arrangements_seen = HashMap::new();
  let mut cycle = None;
  for i in 0..1000000000 {
    let as_str = rocks.get_string_representation();
    let previous_cycle_seen = arrangements_seen.get(&as_str).map(|&n| n);
    if let Some(prev_index) = previous_cycle_seen {
      cycle = Some((prev_index, i));
      break;
    }
    arrangements_seen.insert(as_str.clone(), i);
    rocks.complete_cyle();
  }
  match cycle {
    None => {
      // this hopefully won't happen in practice - it means we've actually gone through all billion cycles.
      // If this somehow happened before the heat-death of the universe (or, less likely, within a few hours),
      // we can just read the answer
      rocks.total_load_north()
    },
    Some((start, end)) => {
      // compute the smallest number of cycles we need to do to get to the same state as
      // after a billion
      let cycle_length = end - start;
      let remaining = (1_000_000_000 - end) % cycle_length;
      for _ in 0..remaining {
        rocks.complete_cyle();
      }
      rocks.total_load_north()
    }
  }
}

pub fn part_2() -> usize {
  let mut rocks = read_file();
  solve_part_2(&mut rocks)
}
