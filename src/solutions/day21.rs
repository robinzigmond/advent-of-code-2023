use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

enum Space {
  Start,
  Garden,
  Rock,
}

fn read_file() -> Vec<Vec<Space>> {
  let mut file = File::open("./input/input21.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  contents.lines().map(|line| {
    line.chars().map(|c| match c {
      'S' => Space::Start,
      '.' => Space::Garden,
      '#' => Space::Rock,
      _ => panic!("unexpected character: {}", c),
    }).collect()
  }).collect()
}

fn find_start(grid: &Vec<Vec<Space>>) -> (usize, usize) {
  for (i, row) in grid.iter().enumerate() {
    for (j, space) in row.iter().enumerate() {
      match space {
        Space::Start => return (i, j),
        _ => {},
      }
    }
  }
  panic!("no start space found!");
}

fn get_garden_neighbours(grid: &Vec<Vec<Space>>, row: usize, col: usize) -> Vec<(usize, usize)> {
  let all_neighbours = [
      (row as isize - 1, col as isize),
      (row as isize + 1, col as isize),
      (row as isize, col as isize - 1),
      (row as isize, col as isize + 1)
    ];
  
  let neighbours = all_neighbours.iter()
    .filter(|(i, j)| *i >= 0 && *j >= 0 && *i < grid.len() as isize && *j < grid[0].len() as isize)
    .map(|(i, j)| (*i as usize, *j as usize));

  neighbours.filter(|(i, j)| match grid[*i][*j] {
    Space::Rock => false,
    Space::Start | Space::Garden => true,
  }).collect()
}

fn get_spaces_after_steps(grid: &Vec<Vec<Space>>, start_space: (usize, usize), num_steps: usize) -> HashSet<(usize, usize)> {
  let mut spaces_found = HashSet::new();
  spaces_found.insert(start_space);
  for _ in 0..num_steps {
    let mut new_spaces = HashSet::new();
    for (row, col) in spaces_found {
      let neighbours = get_garden_neighbours(grid, row, col);
      for neighbour in neighbours {
        new_spaces.insert(neighbour);
      }
    }
    spaces_found = new_spaces;
  }

  spaces_found
}

fn get_spaces_after_steps_from_start(grid: &Vec<Vec<Space>>, num_steps: usize) -> HashSet<(usize, usize)> {
  let start_space = find_start(grid);
  get_spaces_after_steps(grid, start_space, num_steps)
}

fn solve_part_1(grid: &Vec<Vec<Space>>) -> usize {
  get_spaces_after_steps_from_start(grid, 64).len()
}

pub fn part_1() -> usize {
  let grid = read_file();
  solve_part_1(&grid)
}

// note this solution relies on some "nice features" of the grid in the input data, namely that
// all edges are completely clear of rocks, as are the straight horizontal and vertical lines
// through the centre (which is where the start space is located)
fn solve_part_2(grid: &Vec<Vec<Space>>) -> u64 {
  let num_steps = 26501365;
  // note that grid width and grid height are the same
  let grid_size = grid.len();
  let n = num_steps / grid_size as u64;
  // just hard-code this - it's both the distance remaining after going n * 131 to reach one of the furthest grid centres,
  // AND the distance from the centre of the grid to the entre of an edge, or from an edge centre to a corner
  let distance_remaining = 65;
  // the diagram looks something like this, drawn for the very simple case where n = 2. Each letter marks a copy of the whole
  // grid. The starting grid is the I in the centre.
  //
  //  OXO
  // OXIXO
  // XIIIX
  // OXIXO
  //  OXO
  //
  //  Of the marked grids:
  //  The Xs are the furthest grids we can reach the centre of, and then have 65 steps left after reaching the centre.
  //  Note that 65 is not enough (only just) to reach a different grid.
  //  HOWEVER, it is still possible to reach the edges of the grids marked O: from the centre of the nearest I (reached
  //  with 131 + 65 steps remaining), go to just the nearest edge of the X (66 steps) and then along its edge until
  //  reaching the corner of the O (66 more steps) - the corner of O is reached with 64 steps still remaining, and we
  //  can't reach any other part of it quicker than we'd get via the corner.
  //  In general there will be 4n of these O's, divided into 4 sets of n, where we come into the Os in each set by
  //  a different corner.
  //  So we can compute the total contribution from O's as n times the sum over all 4 corners of (number of square
  //  you can reach in 64 from that corner).
  //
  //  What about the Xs and Is?
  //  For the Xs, we can reach the centre with 65 steps remaining - but we will enter them first at an edge centre,
  //  with 65 + 65 = 130 steps still remaining. Each except the 4 at the extreme points (North/East/South/West) will
  //  be reachable from 2 different sides - respectively N/E, N/W, S/E and S/W. So for each such pair of sides we
  //  must work out the total that can be reached in 130 from the midpoint of each side and take a union of these.
  //  Then we sum those quantities up and multiply by the number of X's that are reached by that pair of sides
  //  - which is n - 1. Finally we also have the 4 "extreme" points: each of these can only be reached by one edge,
  //  so we just take the squares we can reach in 130 from each edge-centre and add these on to the total.
  //
  //  Finally we have the Is - which contribute by far the most to the solution, as there are far more of them
  //  and most squares can be reached inside them (as we have more steps remaining).
  //  From most of these, we can assume that there are sufficient steps remaining to reach all reachable spaces.
  //  "reachable" is important here, as there are 2 things in particular that can prevent a square being reached
  //  in a given number of steps, no matter how high. One is if they are "walled in" behind a barrier of rocks on
  //  all 4 sides - there are a few of these patches on the grid!
  //  The other is parity - odd and even numbers of steps remaining give access to a fundamentally different set of
  //  spaces, as for example it's possible to reach the starting space after any even number of steps, but not after
  //  ANY odd number. These two "parity sets" of "odd spaces" and "even spaces" are therefore disjoint, and between
  //  then cover all spaces but the "walled off" ones.
  //  To compute these, it suffices to take a sufficiently large odd/even number and feed that into the standard function.
  //  I won't code it rigorously below, just with values that are reasonably quick to compute and that I know from further
  //  work are correct - much larger even/odd values don't increase the se size, and between them these 2 figures add up to
  //  the total number of reachable spots in the grid (which is all 's bare 16 that are "walled off", mostly individually
  //  but there is one group of 3 as well).
  //  2 questions remain. One is how to count the number of "even parity" and "odd parity" such grids. Walking from one
  //  to an adjacent one changes the parity as there are 131 steps between - an odd number. Therefore they form a
  //  checkerboard pattern. And further the central grid is an "odd" one, just because the starting number of steps is odd.
  //  So to compute the total number of each, we can work out from the centre: start with 1 odd, then add alternately
  //  4 even, then 8 odd, then 12 even, and so on.
  //  The second question is whether we *really* have enough steps remaining, at the outermost I's, to reach ALL spaces
  //  of the correct ("even") parity. This can easily be computed separately, and it turns out the answer is yes - 196
  //  steps (131 + 65) is enough to reach all such spaces.

  let mut num_odd_interior_grids = 1;
  let mut num_even_interior_grids = 0;
  // don't include the limit as those are the X grids that we're covering separately!
  for i in 1..n {
    let boundary_size = 4*i;
    if i % 2 == 1 {
      num_even_interior_grids += boundary_size;
    } else {
      num_odd_interior_grids += boundary_size;
    }
  }

  // this is the count for the I's in the diagram/discussion above
  let odd_spaces_count = get_spaces_after_steps_from_start(grid, 2 * grid_size - 1).len() as u64;
  let even_spaces_count = get_spaces_after_steps_from_start(grid, 2 * grid_size).len() as u64;

  let interior_count = odd_spaces_count * num_odd_interior_grids + even_spaces_count * num_even_interior_grids;

  // now the X's
  let edge_center = (grid_size - 1) / 2;
  let reachable_from_top = get_spaces_after_steps(grid, (0, edge_center), 2 * distance_remaining);
  let reachable_from_bottom = get_spaces_after_steps(grid, (grid_size - 1, edge_center), 2 * distance_remaining);
  let reachable_from_left = get_spaces_after_steps(grid, (edge_center, 0), 2 * distance_remaining);
  let reachable_from_right = get_spaces_after_steps(grid, (edge_center, grid_size - 1), 2 * distance_remaining);
  let top_and_left = reachable_from_top.union(&reachable_from_left);
  let top_and_right = reachable_from_top.union(&reachable_from_right);
  let bottom_and_left = reachable_from_bottom.union(&reachable_from_left);
  let bottom_and_right = reachable_from_bottom.union(&reachable_from_right);

  let edge_count = ((n as usize - 1) * (top_and_left.count() + top_and_right.count() + bottom_and_left.count() + bottom_and_right.count())
                        + (reachable_from_top.len() + reachable_from_bottom.len() + reachable_from_left.len() + reachable_from_right.len())) as u64;

  // now the O's
  let top_left_corner_count = get_spaces_after_steps(grid, (0, 0), distance_remaining - 1).len();
  let top_right_corner_count = get_spaces_after_steps(grid, (0, grid_size - 1), distance_remaining - 1).len();
  let bottom_left_corner_count = get_spaces_after_steps(grid, (grid_size - 1, 0), distance_remaining - 1).len();
  let bottom_right_corner_count = get_spaces_after_steps(grid, (grid_size - 1, grid_size - 1), distance_remaining - 1).len();

  let beyond_edge_count = n * (top_left_corner_count + top_right_corner_count + bottom_left_corner_count + bottom_right_corner_count) as u64;

  // add all together to get the final total
  interior_count + edge_count + beyond_edge_count
}

pub fn part_2() -> u64 {
  let grid = read_file();
  solve_part_2(&grid)
}
