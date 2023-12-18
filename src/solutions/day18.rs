use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

struct DigInstruction {
  direction: Direction,
  distance: isize,
  color: String,
}

fn read_line(l: &str) -> DigInstruction {
  let parts: Vec<&str> = l.split(" ").collect();
  let direction = match parts[0] {
    "U" => Direction::Up,
    "D" => Direction::Down,
    "L" => Direction::Left,
    "R" => Direction::Right,
    s => panic!("unexpected instruction: {}", s),
  };
  let distance = parts[1].parse().unwrap();
  let mut color = parts[2].to_owned();
  // remove the ( and ) from the ends
  color.remove(0);
  color.remove(color.chars().count() - 1);

  DigInstruction { direction, distance, color }
}

// this is an "annotated" form of the instructions which doesn't require us
// to follow them through in order in order to know where they are
#[derive(PartialEq, Eq, Hash)]
struct AnnotatedInstruction {
  direction: Direction,
  start: isize,
  finish: isize,
  position: isize,
}

fn annotate_instructions(instructions: &Vec<DigInstruction>) -> HashSet<AnnotatedInstruction> {
  let mut result = HashSet::new();

  let mut current_row = 0;
  let mut current_column = 0;

  for instruction in instructions {
    let DigInstruction { direction, distance, color: _ } = instruction;
    let with_annotation = match direction {
      Direction::Up => {
        current_row -= distance;
        AnnotatedInstruction {
          direction: Direction::Up,
          start: current_row,
          finish: current_row + distance,
          position: current_column,
        }
      },
      Direction::Down => {
        current_row += distance;
        AnnotatedInstruction {
          direction: Direction::Down,
          start: current_row - distance,
          finish: current_row,
          position: current_column,
        }
      },
      Direction::Left => {
        current_column -= distance;
        AnnotatedInstruction {
          direction: Direction::Left,
          start: current_column,
          finish: current_column + distance,
          position: current_row,
        }
      },
      Direction::Right => {
        current_column += distance;
        AnnotatedInstruction {
          direction: Direction::Right,
          start: current_column - distance,
          finish: current_column,
          position: current_row,
        }
      },
    };
    result.insert(with_annotation);
  }

  result
}

fn get_trench_coords(instructions: &Vec<DigInstruction>) -> HashSet<(isize, isize)> {
  let mut result = HashSet::new();
  result.insert((0, 0));
  let mut current_row = 0;
  let mut current_column = 0;

  for instruction in instructions {
    let DigInstruction { direction, distance, color: _ } = instruction;
    for _ in 1..=*distance {
      match direction {
        Direction::Up => current_row -= 1,
        Direction::Down => current_row += 1,
        Direction::Left => current_column -= 1,
        Direction::Right => current_column += 1,
      }
      result.insert((current_row, current_column));
    }
  }

  result
}

fn read_file() -> Vec<DigInstruction> {
  let mut file = File::open("./input/input18.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  contents.lines().map(read_line).collect()
}

fn solve_part_1(instructions: Vec<DigInstruction>) -> u64 {
  // solve this the same way as day 10, part 2. In order to count crossing more easily, we use
  // the "annotated" instructions.

  // We also get the coordinates of all points actually dug - this makes it easier to get the bounds of the grid.
  // (While it's a little inefficient to do this, we only have to do it once so it should be OK)
  let trench_coords = get_trench_coords(&instructions);

  let min_width = *trench_coords.iter().map(|(_, y)| y).min().unwrap();

  // count all the trench pieces first - the below calculations will be careful not to double-count them!
  let mut count = trench_coords.len() as u64;

  let annotated = annotate_instructions(&instructions);

  // for part 2 it takes far too long to iterate one-by-one through either the rows or the columns.
  // To deal with the rows, we have to split them up into consecutive groups of rows with "the same pattern",
  // so we only compute these values once and then multiply them by the group size.
  // What in the "annotated" data would make 2 rows "the same"?
  // For 2 specific rows i and j, two things are both needed make them the same:
  // - the left/right groups with position = i and position = j are identical (in start and finish points - the directions
  // could be opposite without making a difference)
  // - the up/down groups never include just one of i/j between their start and finish: they all have either both or neither
  // It's easiest to start by dealing with the second of these criteria, as it eliminates a lot more (most if not all
  // row-pairs that are "the same" under these criteria will have NO left/right groups in them at all!).
  // For this a crude approach is as follows: find all up/down groups, and form an ordered list of all their start
  // AND end points (which are start and which are end won't actually matter). Each interval between two of these
  // (EXCLUSIVE intervals in this case) must be identical as a set. (This leaves each endpoint to be done individually,
  // but that should be OK as there won't be that many.)
  // Then for the first point, we should actually be able to ignore it at this point - this is because any horizontal
  // boundary segment must correspond to an endpoint of a vertical segment, and we've specifically left these off.
  // So we already have the sets we need, and can compute what happens at each of those, multiply it by their size,
  // and then add on the endpoint rows to work out individually.
  // [This ran for around 30 seconds in release mode - not great, but good enough for me not to want to bother
  // improving it when it's not obvious to me how!]

  let mut all_vertical_endpoints: Vec<isize> = annotated.iter().filter(
    |instruction| vec![Direction::Up, Direction::Down].contains(&instruction.direction)
  ).flat_map(|instr| vec![instr.start, instr.finish]).collect();
  all_vertical_endpoints.sort_unstable();
  all_vertical_endpoints.dedup();

  // going to now build a single vector, whose elements are pairs of the row number and the number of rows
  // that fit the same pattern as it (which will be 1 for all the endpoints we've just retrieved, but much
  // more for points in-betwee - which we'll get by adding 1 to each endpoint, removing the last, and removing
  // any more which may already have been accounted for)
  let mut all_endpoints: Vec<(isize, isize)> = all_vertical_endpoints.iter().map(|&point| (point, 1)).collect();
  for i in 0..(all_vertical_endpoints.len() - 1) {
    let new_endpoint = all_vertical_endpoints[i] + 1;
    let next_endpoint = all_vertical_endpoints[i + 1];
    if next_endpoint > new_endpoint {
      let size = next_endpoint - new_endpoint;
      all_endpoints.push((new_endpoint, size));
    }
  }

  for (vertical_endpoint, factor) in all_endpoints {
    // first add on its own individual contribution:

    // pull out all the vertical pieces which we can cross on this row
    let will_intersect = annotated.iter().filter(
      |instruction| vec![Direction::Up, Direction::Down].contains(&instruction.direction)
        && instruction.start < vertical_endpoint && instruction.finish >= vertical_endpoint
    );
    let mut intersect_positions: Vec<isize> = will_intersect.map(|instr| instr.position).collect();
    intersect_positions.sort_unstable();

    // we also need any horizontal pieces which lie in this row!
    let mut in_row: Vec<Vec<isize>> = annotated.iter().filter(
      |instruction| vec![Direction::Left, Direction::Right].contains(&instruction.direction)
        && instruction.position == vertical_endpoint
    ).map(|instr| vec![instr.start, instr.finish]).collect();
    in_row.sort_unstable();
  
    let mut previous = min_width;
    let mut is_inside = false;
    for position in intersect_positions {
      let mut position_to_use = position;
      // if the previous point was inside a trench interval, skip anything that was inside it (they're already counted
      // above), by jumping previous to the end of the interval)
      if let Some(endpoints) = in_row.iter().find(|endpoints| endpoints[0] <= previous && endpoints[1] >= previous) {
        previous = endpoints[1];
      }
      // same with current point and start of the interval
      if let Some(endpoints) = in_row.iter().find(|endpoints| endpoints[0] <= position && endpoints[1] >= position) {
        position_to_use = endpoints[0];
      }
      if is_inside {
        if previous < position_to_use {
          let mut to_add = position_to_use - previous - 1;
          // we also need to take into acccount any trench intervals that fall entirely between the previous and current
          // positions, and remove their total length. This can happen when both bend "the wrong way" to be counted
          // in intersect_positions.
          for internal_endpoints in in_row.iter().filter(|endpoints| endpoints[0] > previous && endpoints[1] < position_to_use) {
            to_add -= internal_endpoints[1] + 1 - internal_endpoints[0];
          }
          count += (to_add * factor) as u64;
        }
      }
      is_inside = !is_inside;
      previous = position_to_use;
    }
  }

  count
}

pub fn part_1() -> u64 {
  let instructions = read_file();
  solve_part_1(instructions)
}

fn convert_hex(instructions: Vec<DigInstruction>) -> Vec<DigInstruction> {
  instructions.iter().map(|instruction| {
    let color = &instruction.color;
    let all_chars: Vec<char> = color.chars().collect();
    let direction = match all_chars[6] {
      '0' => Direction::Right,
      '1' => Direction::Down,
      '2' => Direction::Left,
      '3' => Direction::Up,
      c => panic!("unexpected final hex digit: {}", c),
    };
    let distance = isize::from_str_radix(&all_chars.split_at(1).1.split_at(5).0.iter().collect::<String>(), 16).unwrap();

    DigInstruction { direction, distance, color: String::new() } // color is irrelevant now
  }).collect()
}

fn solve_part_2(instructions: Vec<DigInstruction>) -> u64 {
  solve_part_1(convert_hex(instructions))
}

pub fn part_2() -> u64 {
  let instructions = read_file();
  solve_part_2(instructions)
}
