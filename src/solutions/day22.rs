use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, PartialEq, Eq)]
struct Brick {
  spaces: Vec<(usize, usize, usize)>,
}

#[derive(Clone)]
struct AllSpace {
  bricks: Vec<Brick>,
}

impl AllSpace {
  fn get_occupied_spaces(&self) -> HashSet<(usize, usize, usize)> {
    let mut occupied = HashSet::new();

    for brick in &self.bricks {
      for space in &brick.spaces {
        occupied.insert(*space);
      }
    }

    occupied
  }

  // makes all bricks fall as far as they can go, and returns a vector of all bricks which
  // ended up falling. (This complete vector is needed for part 2 to work properly.)
  // In fact to make part 2 easier we return both the starting AND finishing position of each
  // brick that falls.
  // Note: this is dependent on the order the bricks are stored in the vector, which shouldn't
  // in principle matter - however when done repeatly in the settle_all_bricks method below,
  // it becomes reliable.
  fn fall_all_bricks(&mut self) -> Vec<(Brick, Brick)> {
    let mut occupied = self.get_occupied_spaces();
    let mut fallen_bricks = vec![];
    for brick in &mut self.bricks {
      let mut can_this_fall = true;
      let mut already_marked = false;
      let brick_start_position = brick.clone();
      let brick_is_vertical = brick.spaces.len() > 1 && brick.spaces[0].2 != brick.spaces[1].2;
      while can_this_fall {
        if brick_is_vertical {
          // we know the first space will have the lowest z co-ordinate - this is the one we have to check
          // the space below for occupancy
          let (x, y, z) = brick.spaces[0];
          if z == 1 || occupied.contains(&(x, y, z - 1)) {
            can_this_fall = false;
          }
        } else {
          for &(x, y, z) in &brick.spaces {
            if z == 1 || occupied.contains(&(x, y, z - 1)) {
              can_this_fall = false;
              break;
            }
          }
        }
        if can_this_fall {
          if already_marked {
            fallen_bricks.pop();
          }
          already_marked = true;
          for space in &mut brick.spaces {
            space.2 -= 1;
            // also need to update occupied spaces - easiest to do manually
            occupied.remove(&(space.0, space.1, space.2 + 1));
            occupied.insert(*space);
          }
          fallen_bricks.push((brick_start_position.clone(), brick.clone()));
        }
      }
    }
    fallen_bricks
  }

  fn settle_all_bricks(&mut self) {
    let mut all_settled = false;
    while !all_settled {
      all_settled = self.fall_all_bricks().len() == 0;
    }
  }

  fn disintegratable_bricks(&self) -> Vec<&Brick> {
    self.bricks.iter().filter(|brick| {
      let mut copy = self.clone();
      copy.bricks = copy.bricks.iter().filter(|b| b != brick).map(|b| b.to_owned()).collect();
      copy.fall_all_bricks().len() == 0
    }).collect()
  }
}

fn parse_brick(line: &str) -> Brick {
  let parts: Vec<&str> = line.split("~").collect();
  let first_end: Vec<usize> = parts[0].split(",").map(|n| n.parse().unwrap()).collect();
  let second_end: Vec<usize> = parts[1].split(",").map(|n| n.parse().unwrap()).collect();

  let spaces = if first_end[0] < second_end[0] {
    (first_end[0]..=second_end[0]).map(|x| (x, first_end[1], first_end[2])).collect()
  } else if first_end[1] < second_end[1] {
    (first_end[1]..=second_end[1]).map(|y| (first_end[0], y, first_end[2])).collect()
  } else if first_end[2] < second_end[2] {
    (first_end[2]..=second_end[2]).map(|z| (first_end[0], first_end[1], z)).collect()
  } else if first_end == second_end {
    vec![(first_end[0], first_end[1], first_end[2])]
  } else {
    panic!("unexpected brick pattern: {}", line)
  };

  Brick { spaces }
}

fn read_file() -> AllSpace {
  let mut file = File::open("./input/input22.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let bricks = contents.lines().map(parse_brick).collect();

  AllSpace { bricks }
}

fn solve_part_1(space: &mut AllSpace) -> usize {
  space.settle_all_bricks();
  space.disintegratable_bricks().len()
}

pub fn part_1() -> usize {
  let mut space = read_file();
  solve_part_1(&mut space)
}

fn solve_part_2(space: &mut AllSpace) -> u32 {
  space.settle_all_bricks();
  let mut result = 0;
  for brick in &space.bricks {
    // need to take a copy so that we can keep reverting to this "settled state".
    // Unfortunately this has to be done inside the loop so that each brick can revert
    // to this.
    let mut settled_state = space.clone();
    settled_state.bricks = settled_state.bricks.iter().filter(|&b| b != brick).map(|b| b.to_owned()).collect();
    let mut fall_count = 0;
    let mut continue_falling = true;
    let mut just_fell = vec![];
    while continue_falling {
      let now_falling = settled_state.fall_all_bricks();
      // we don't want to count anything that is still falling from the previous step
      let num_new_fallers = now_falling.iter().filter(
        |(start_pos, _)| just_fell.iter().find(|(_, end_pos)| end_pos == start_pos).is_none()
      ).count();
      fall_count += num_new_fallers;
      just_fell = now_falling.into_iter().collect();
      continue_falling = num_new_fallers > 0;
    }
    result += fall_count as u32;
  }
  result
}

pub fn part_2() -> u32 {
  let mut space = read_file();
  // sort by increasing z-value as that will make everything easier!
  // (The answer I get without it is wrong, while with it it's right - I'm not entirely sure why)
  space.bricks.sort_by(
    |Brick { spaces: spaces1 }, Brick { spaces: spaces2}|
    spaces1[0].2.cmp(&spaces2[0].2)
  );
  solve_part_2(&mut space)
}
