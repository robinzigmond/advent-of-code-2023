use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
enum Mirror {
  ReflectorForward,
  ReflectorBackward,
  SplitterVertical,
  SplitterHorizontal,
}

fn read_file() -> Vec<Vec<Option<Mirror>>> {
  let mut file = File::open("./input/input16.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut grid = vec![];
  for line in contents.lines() {
    let mut row = vec![];
    for c in line.chars() {
      let possible_mirror = match c {
        '.' => None,
        '/' => Some(Mirror::ReflectorForward),
        '\\' => Some(Mirror::ReflectorBackward),
        '|' => Some(Mirror::SplitterVertical),
        '-' => Some(Mirror::SplitterHorizontal),
        _ => panic!("unexpected input character: {}", c),
      };
      row.push(possible_mirror);
    }
    grid.push(row);
  }

  grid
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
  North,
  South,
  East,
  West,
}

struct BeamTracer {
  grid: Vec<Vec<Option<Mirror>>>,
  beams: HashMap<(usize, usize), HashSet<Direction>>,
}

impl BeamTracer {
  // used to remove repetition in the "main" move_from function below
  fn go_in_directions(&mut self, row: usize, col: usize, directions: Vec<Direction>) {
    for direction in directions {
      match direction {
        Direction::North => {
          if row > 0 {
            self.move_from(row - 1, col, direction);
          }
        },
        Direction::South => {
          if row < self.grid.len() - 1 {
            self.move_from(row + 1, col, direction);
          }
        },
        Direction::East => {
          if col < self.grid[0].len() - 1 {
            self.move_from(row, col + 1, direction);
          }
        },
        Direction::West => {
          if col > 0 {
            self.move_from(row, col - 1, direction);
          }
        },
      }
    }
  }

  fn move_from(&mut self, row: usize, col: usize, direction: Direction) {
    let current_directions = self
      .beams
      .get_mut(&(row, col));

    match current_directions {
      Some(directions) => {
        // the insert method returns a boolean indicating if it was actually inserted or not:
        // if it failed to do so because we've already been to this position going the same
        // direction, there's no need to continue.
        if !directions.insert(direction) {
          return;
        }
      },
      None => {
        let mut directions = HashSet::new();
        directions.insert(direction);
        self.beams.insert((row, col), directions);
      },
    }

    let location_contents = self.grid[row][col];

    match location_contents {
      Some(Mirror::ReflectorForward) => {
        let new_direction = match direction {
          Direction::North => Direction::East,
          Direction::South => Direction::West,
          Direction::East => Direction::North,
          Direction::West => Direction::South,
        };
        self.go_in_directions(row, col, vec![new_direction]);
      },
      Some(Mirror::ReflectorBackward) => {
        let new_direction = match direction {
          Direction::North => Direction::West,
          Direction::South => Direction::East,
          Direction::East => Direction::South,
          Direction::West => Direction::North,
        };
        self.go_in_directions(row, col, vec![new_direction]);
      },
      Some(Mirror::SplitterHorizontal) => {
        let new_directions = match direction {
          Direction::North | Direction::South => vec![Direction::East, Direction::West],
          Direction::East | Direction::West => vec![direction],
        };
        self.go_in_directions(row, col, new_directions); 
      },
      Some(Mirror::SplitterVertical) => {
        let new_directions = match direction {
          Direction::North | Direction::South => vec![direction],
          Direction::East | Direction::West => vec![Direction::North, Direction::South],
        };
        self.go_in_directions(row, col, new_directions); 
      },
      None => {
        self.go_in_directions(row, col, vec![direction]); 
      },
    }
  }
}

fn solve_part_1(grid: Vec<Vec<Option<Mirror>>>) -> usize {
  let mut tracer = BeamTracer { grid, beams: HashMap::new() };
  tracer.move_from(0, 0, Direction::East);
  tracer.beams.keys().len()
}

pub fn part_1() -> usize {
  let grid = read_file();
  solve_part_1(grid)
}

fn solve_part_2(grid: Vec<Vec<Option<Mirror>>>) -> usize {
  let mut maximum = 0;
  // test left edge going East
  for i in 0..grid.len() {
    let mut tracer = BeamTracer { grid: grid.clone(), beams: HashMap::new() };
    tracer.move_from(i, 0, Direction::East);
    let result = tracer.beams.keys().len();
    if result > maximum {
      maximum = result;
    }
  }
  // test right edge going West
  for i in 0..grid.len() {
    let mut tracer = BeamTracer { grid: grid.clone(), beams: HashMap::new() };
    tracer.move_from(i, grid[0].len() - 1, Direction::West);
    let result = tracer.beams.keys().len();
    if result > maximum {
      maximum = result;
    }
  }
  // test top edge going South
  for i in 0..grid[0].len() {
    let mut tracer = BeamTracer { grid: grid.clone(), beams: HashMap::new() };
    tracer.move_from(0, i, Direction::South);
    let result = tracer.beams.keys().len();
    if result > maximum {
      maximum = result;
    }
  }
  // test bottom edge going North
  for i in 0..grid[0].len() {
    let mut tracer = BeamTracer { grid: grid.clone(), beams: HashMap::new() };
    tracer.move_from(grid.len() - 1, i, Direction::South);
    let result = tracer.beams.keys().len();
    if result > maximum {
      maximum = result;
    }
  }
  maximum
}

pub fn part_2() -> usize {
  let grid = read_file();
  solve_part_2(grid)
}
