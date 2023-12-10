use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy)]
enum Tile {
  PipeVertical,
  PipeHorizontal,
  PipeNorthEast,
  PipeNorthWest,
  PipeSouthWest,
  PipeSouthEast,
  Ground,
  Start,
}

struct Grid {
  tiles: Vec<Vec<Tile>>
}

impl Grid {
  fn get_tile(&self, row: usize, column: usize) -> Tile {
    self.tiles[row][column]
  }
}

enum Direction {
  North,
  East,
  South,
  West,
}

fn read_file() -> Grid {
  let mut file = File::open("./input/input10.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut tiles = vec![];
  for row in contents.lines() {
    let mut pipes = vec![];
    for char in row.chars() {
      let tile = match char {
        '|' => Tile::PipeVertical,
        '-' => Tile::PipeHorizontal,
        'L' => Tile::PipeNorthEast,
        'J' => Tile::PipeNorthWest,
        '7' => Tile::PipeSouthWest,
        'F' => Tile::PipeSouthEast,
        '.' => Tile::Ground,
        'S' => Tile::Start,
        _ => panic!("unknown tile: {}", char),
      };
      pipes.push(tile);
    }
    tiles.push(pipes);
  }

  Grid { tiles }
}

fn follow_path(grid: &Grid, row: usize, column: usize, incoming_direction: &Option<Direction>) -> Direction {
  match grid.get_tile(row, column) {
    Tile::PipeVertical => {
      match incoming_direction {
        Some(Direction::North) => return Direction::North,
        Some(Direction::South) => return Direction::South,
        _ => panic!("came East or West into a vertical pipe?!"),
      }
    },
    Tile::PipeHorizontal => {
      match incoming_direction {
        Some(Direction::East) => return Direction::East,
        Some(Direction::West) => return Direction::West,
        _ => panic!("came North or South into a horizontal pipe?!"),
      }
    },
    Tile::PipeNorthEast => {
      match incoming_direction {
        Some(Direction::South) => return Direction::East,
        Some(Direction::West) => return Direction::North,
        _ => panic!("came North or East into a North/East pipe?!"),
      }
    },
    Tile::PipeNorthWest => {
      match incoming_direction {
        Some(Direction::South) => return Direction::West,
        Some(Direction::East) => return Direction::North,
        _ => panic!("came North or West into a North/West pipe?!"),
      }
    },
    Tile::PipeSouthWest => {
      match incoming_direction {
        Some(Direction::North) => return Direction::West,
        Some(Direction::East) => return Direction::South,
        _ => panic!("came South or West into a South/West pipe?!"),
      }
    },
    Tile::PipeSouthEast => {
      match incoming_direction {
        Some(Direction::North) => return Direction::East,
        Some(Direction::West) => return Direction::South,
        _ => panic!("came South or East into a South/East pipe?!"),
      }
    },
    Tile::Start => {
      // we need to determine a direction to start off in. Just cycle through the possible starting locations and
      // stop at the first one that gives us a connecting pipe
      if row > 0 {
        let north_tile = grid.get_tile(row - 1, column);
        match north_tile {
          Tile::PipeVertical | Tile::PipeSouthEast | Tile::PipeSouthWest => {
            return Direction::North;
          },
          _ => {},
        }
      }
      if column < grid.tiles[0].len() - 1 {
        let east_tile = grid.get_tile(row, column + 1);
        match east_tile {
          Tile::PipeHorizontal | Tile::PipeNorthWest | Tile::PipeSouthWest => {
            return Direction::East;
          },
          _ => {},
        }
      }
      if row < grid.tiles.len() - 1 {
        let south_tile = grid.get_tile(row + 1, column);
        match south_tile {
          Tile::PipeVertical | Tile::PipeNorthWest | Tile::PipeNorthEast => {
            return Direction::South;
          },
          _ => {},
        }
      }
      if column > 0 {
        let west_tile = grid.get_tile(row, column - 1);
        match west_tile {
          Tile::PipeHorizontal | Tile::PipeNorthEast | Tile::PipeSouthEast => {
            return Direction::West;
          },
          _ => {},
        }
      }
      panic!("can't go any direction from start tile??");
    },
    Tile::Ground => panic!("we hit a ground tile while following poipes??"),
  }
}

fn get_start_tile(grid: &Grid) -> (usize, usize) {
  for (row_index, row) in grid.tiles.iter().enumerate() {
    for (col_index, tile) in row.iter().enumerate() {
      if let Tile::Start = tile {
        return (row_index, col_index);
      }
    }
  }
  panic!("couldn't find start tile!");
}

fn solve_part_1(grid: &Grid) -> u32 {
  let (start_row, start_col) = get_start_tile(grid);
  let mut current_row = start_row;
  let mut current_col = start_col;
  let mut next_direction = None;
  let mut total_steps = 0;

  loop {
    total_steps += 1;
    next_direction = Some(follow_path(grid, current_row, current_col, &next_direction));
    match next_direction {
      Some(Direction::North) => current_row -= 1,
      Some(Direction::South) => current_row += 1,
      Some(Direction::West) => current_col -= 1,
      Some(Direction::East) => current_col += 1,
      None => panic!("can't happen, value was just set explicitly to a Some!"),
    };
    let current_tile = grid.get_tile(current_row, current_col);
    if let Tile::Start = current_tile {
      return total_steps / 2;
    }
  }
}

pub fn part_1() -> u32 {
  let grid = read_file();
  solve_part_1(&grid)
}

// basically a repeat of the part 1 solution, but compiling a list of all the points traversed,
// because we'll need all these for part 2
fn get_loop_path(grid: &Grid) -> Vec<(usize, usize)> {
  let mut loop_tiles = vec![];
  let (start_row, start_col) = get_start_tile(grid);
  let mut current_row = start_row;
  let mut current_col = start_col;
  let mut next_direction = None;

  loop {
    next_direction = Some(follow_path(grid, current_row, current_col, &next_direction));
    match next_direction {
      Some(Direction::North) => current_row -= 1,
      Some(Direction::South) => current_row += 1,
      Some(Direction::West) => current_col -= 1,
      Some(Direction::East) => current_col += 1,
      None => panic!("can't happen, value was just set explicitly to a Some!"),
    };
    let current_tile = grid.get_tile(current_row, current_col);
    loop_tiles.push((current_row, current_col));
    if let Tile::Start = current_tile {
      return loop_tiles;
    }
  }
}

// for part 2 we actually need to know which type of corner the start tile is, which we avoided computing before!
fn get_start_type(grid: &Grid) -> Tile {
  let (row, col) = get_start_tile(grid);
  let mut can_go_north = false;
  let mut can_go_south = false;
  let mut can_go_east = false;
  let mut can_go_west = false;
  // this main logic is copied from the follow_path function, where we already had it but not quite in
  // a form that's easy to reuse
  if row > 0 {
    let north_tile = grid.get_tile(row - 1, col);
    match north_tile {
      Tile::PipeVertical | Tile::PipeSouthEast | Tile::PipeSouthWest => {
        can_go_north = true;
      },
      _ => {},
    }
  }
  if col < grid.tiles[0].len() - 1 {
    let east_tile = grid.get_tile(row, col + 1);
    match east_tile {
      Tile::PipeHorizontal | Tile::PipeNorthWest | Tile::PipeSouthWest => {
        can_go_east = true;
      },
      _ => {},
    }
  }
  if row < grid.tiles.len() - 1 {
    let south_tile = grid.get_tile(row + 1, col);
    match south_tile {
      Tile::PipeVertical | Tile::PipeNorthWest | Tile::PipeNorthEast => {
        can_go_south = true;
      },
      _ => {},
    }
  }
  if col > 0 {
    let west_tile = grid.get_tile(row, col - 1);
    match west_tile {
      Tile::PipeHorizontal | Tile::PipeNorthEast | Tile::PipeSouthEast => {
        can_go_west = true;
      },
      _ => {},
    }
  }

  if can_go_north && can_go_east {
    return Tile::PipeNorthEast;
  }
  if can_go_north && can_go_west {
    return Tile::PipeNorthWest;
  }
  if can_go_south && can_go_east {
    return Tile::PipeSouthEast;
  }
  if can_go_south && can_go_west {
    return Tile::PipeSouthWest;
  }
  if can_go_north && can_go_south {
    return Tile::PipeVertical;
  }
  if can_go_east && can_go_west {
    return Tile::PipeHorizontal;
  }
  panic!("no valid pipe found for start tile!");
}

fn solve_part_2(grid: &Grid) -> u32 {
  // the approach is as follows. Go down each row, counting the number of times
  // we cross a pipe tile that's in the loop. The idea is to keep track of how many
  // times we've crossed the loop - where this is odd, we must be inside the loop, and
  // if even, we're outside.
  // Note that, since we're traversing horizontally, encountering a vertical pipe tile means
  // we have definitely crossed the loop, while a horizontal does not - we're merely following
  // it for a bit.
  // The trickier case is the corner tiles. As we go from West to East, when we first encounter one it
  // must be a North/East or South/East tile (because we weren't following the path to start with), and
  // it leads us on to the path. Then when we exit, which we must via a North/West or South/West, it's the
  // relative North-South directions of the "entrance" and "exit" corners that matter - if they are both the
  // same, the path has "bent" the same way on entrance and exit so we haven't crossed the path (the section
  // of path we just followed could have been shiften just one unit and we wouldn't have seen it at atll).
  // Where if we enter at a North/East tile and exit at a South/West one, or vice versa, we have moved from
  // the South side to the North side (or vice versa) of the local section of path, so have crossed it.
  let loop_tiles = get_loop_path(grid);
  let mut inside_tiles = 0;
  for row in 0..grid.tiles.len() {
    let mut is_inside = false;
    let mut join_direction = None;
    for col in 0..grid.tiles[row].len() {
      if loop_tiles.contains(&(row, col)) {
        let mut current_tile = grid.get_tile(row, col);
        if let Tile::Start = current_tile {
          current_tile = get_start_type(grid);
        }
        match current_tile {
          Tile::PipeVertical => {
            is_inside = !is_inside;
          },
          Tile::PipeNorthEast => {
            join_direction = Some(Direction::North);
          },
          Tile::PipeSouthEast => {
            join_direction = Some(Direction::South);
          },
          Tile::PipeSouthWest => {
            if let Some(Direction::North) = join_direction {
              is_inside = !is_inside;
            }
          },
          Tile::PipeNorthWest => {
            if let Some(Direction::South) = join_direction {
              is_inside = !is_inside;
            }
          }
          _ => (),
        }
      } else if is_inside {
        inside_tiles += 1;
      }
    }
  }
  inside_tiles
}

pub fn part_2() -> u32 {
  let grid = read_file();
  solve_part_2(&grid)
}
