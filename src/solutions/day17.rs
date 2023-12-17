use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_file() -> Vec<Vec<u32>> {
  let mut file = File::open("./input/input17.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut grid = vec![];
  for line in contents.lines() {
    let mut row = vec![];
    for c in line.chars() {
      row.push(c.to_string().parse().unwrap());
    }
    grid.push(row);
  }

  grid
}

// going to use Djikstra's algorithm to find the best path (https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
// To avoid having to commit to a particular implementation of the priority queue early, going to use a trait
// with the necessary methods, that will be all we use in the main algorithm - then I can more easily swap the
// implementation for a different one where needed.

trait MinPriorityQueue<T: Copy> {
  fn add_node(&mut self, node: T, priority: u32);

  fn decrease_priority(&mut self, node: T, new_priority: u32);

  fn extract_minimum(&mut self) -> (T, u32);

  fn get_priority(&self, node: T) -> Option<u32>;

  // convenience method that either adds or updates as necessary
  fn add_or_update_node(&mut self, node: T, priority: u32) {
    let old_priority = self.get_priority(node);
    match old_priority {
      None => self.add_node(node, priority),
      Some(old) => {
        if priority < old {
          self.decrease_priority(node, priority);
        }
      },
    }
  }

  // not strictly part of the priority queue implementation, but convenient for writing a
  // generic algorithm!
  fn new() -> Self;
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
  North,
  South,
  East,
  West,
}

// this is the node type we'll use. It's essentially a combination of (row, col) co-ordinates with the direction
// we're going in - because the direction determines which nodes we can actually get to next.
// However, we store the start and end as special cases, because in neither of these cases does direction matter
// and, more importantly, when we "end" node signifies that we can stop, once that becomes the "current node".
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Node {
  Start,
  End,
  Other(usize, usize, Direction),
}

// the most naive possible implementation. Likely won't work well because extract_minimum is O(n). But it's
// the simplest to code and may work for the test case, making it easier to check the algorithm has been
// implemented correctly.
// [This worked fine for both parts - even part 2 only 12 seconds on the real data when compiled in release mode.
//  So I never bothered implementing it more efficiently!]
struct HashmapQueue(HashMap<Node, u32>);

impl MinPriorityQueue<Node> for HashmapQueue {
  fn add_node(&mut self, node: Node, priority: u32) {
    self.0.insert(node, priority);
  }

  fn decrease_priority(&mut self, node: Node, new_priority: u32) {
    self.0.insert(node, new_priority);
  }

  fn extract_minimum(&mut self) -> (Node, u32) {
    let mut min: Option<(Node, u32)> = None;

    for (node, distance) in self.0.clone() {
      if min.is_none() || distance < min.unwrap().1 {
        min = Some((node, distance))
      }
    }

    self.0.remove(&min.unwrap().0);

    min.unwrap()
  }

  fn get_priority(&self, node: Node) -> Option<u32> {
    self.0.get(&node).map(|&n| n)
  }

  fn new() -> Self {
    Self(HashMap::new())
  }
}

// a helper function which makes the expression of the main algorithm a lot nicer, particularly in avoiding
// repetition between parts 1 and 2!
fn move_in_direction(queue: &mut impl MinPriorityQueue<Node>, grid: &Vec<Vec<u32>>, current_row: usize, current_col: usize, direction: Direction, min_distance: usize, max_distance: usize, current_min: u32) {
  match direction {
    Direction::North => {
      let mut new_distance = current_min;
      // need to start all these loops from 1 to ensure the distances are correct, even though
      // the earlier points won't get added to the queue
      for i in 1..=max_distance {
        if current_row >= i {
          new_distance += grid[current_row - i][current_col];
          if i >= min_distance {
            let new_node = if current_row == i && current_col == 0 { Node::Start } else { Node::Other(current_row - i, current_col, Direction::North) };
            queue.add_or_update_node(new_node, new_distance);
          }
        }
      }
    },
    Direction::South => {
      let mut new_distance = current_min;
      for i in 1..=max_distance {
        if current_row <= grid.len() - i - 1 {
          new_distance += grid[current_row + i][current_col];
          if i >= min_distance {
            let new_node = if current_row == grid.len() - i - 1 && current_col == grid[0].len() - 1 { Node::End } else { Node::Other(current_row + i, current_col, Direction::South) };
            queue.add_or_update_node(new_node, new_distance);
          }
        }
      }
    },
    Direction::East => {
      let mut new_distance = current_min;
      for i in 1..=max_distance {
        if current_col <= grid[0].len() - i - 1 {
          new_distance += grid[current_row][current_col + i];
          if i >= min_distance {
            let new_node = if current_col == grid[0].len() - i - 1 && current_row == grid.len() - 1 { Node::End } else { Node::Other(current_row, current_col + i, Direction::East) };
            queue.add_or_update_node(new_node, new_distance);
          }
        }
      }
    },
    Direction::West => {
      let mut new_distance = current_min;
      for i in 1..=max_distance {
        if current_col >= i {
          new_distance += grid[current_row][current_col - i];
          if i >= min_distance {
            let new_node = if current_col == i && current_row == 0 { Node::Start } else { Node::Other(current_row, current_col - i, Direction::West) };
            queue.add_or_update_node(new_node, new_distance);
          }
        }
      }
    },
  }
}

// the general form of the algorithm, using a generic priority queue implementation.
// Takes as arguments the min and max distance the "crucibles" can travel in, so this can be used
// for both parts of the problem
fn solve_with_djikstra<Q: MinPriorityQueue<Node>>(grid: Vec<Vec<u32>>, min_distance: usize, max_distance: usize) -> u32 {
  let mut queue = Q::new();
  queue.add_node(Node::Start, 0);
  let mut current = (Node::Start, 0);
  let mut visited = vec![];

  loop {
    let (current_node, current_min) = current;
    visited.push(current_node);
    let valid_directions = match current_node {
      Node::End => break,
      Node::Start => vec![Direction::East, Direction::South],
      Node::Other(_, _, dir) => match dir {
        Direction::North | Direction::South => vec![Direction::East, Direction::West],
        Direction::East | Direction::West => vec![Direction::North, Direction::South],
      }
    };

    // add the nodes that we can validly get to with their new priorities
    if valid_directions.contains(&Direction::North) {
      match current_node {
        Node::End => panic!("by design this cannot happen!"),
        Node::Start => (), // can't go North from the start
        Node::Other(row, col, _) => {
          move_in_direction(&mut queue, &grid, row, col, Direction::North, min_distance, max_distance, current_min)
        }
      }
    }
    if valid_directions.contains(&Direction::South) {
      match current_node {
        Node::End => panic!("by design this cannot happen!"),
        Node::Start => {
          move_in_direction(&mut queue, &grid, 0, 0, Direction::South, min_distance, max_distance, current_min)
        },
        Node::Other(row, col, _) => {
          move_in_direction(&mut queue, &grid, row, col, Direction::South, min_distance, max_distance, current_min)
        }
      }
    }
    if valid_directions.contains(&Direction::East) {
      match current_node {
        Node::End => panic!("by design this cannot happen!"),
        Node::Start => {
          move_in_direction(&mut queue, &grid, 0, 0, Direction::East, min_distance, max_distance, current_min)
        },
        Node::Other(row, col, _) => {
          move_in_direction(&mut queue, &grid, row, col, Direction::East, min_distance, max_distance, current_min)
        }
      }
    }
    if valid_directions.contains(&Direction::West) {
      match current_node {
        Node::End => panic!("by design this cannot happen!"),
        Node::Start => (), // can't go West from the start
        Node::Other(row, col, _) => {
          move_in_direction(&mut queue, &grid, row, col, Direction::West, min_distance, max_distance, current_min)
        }
      }
    }

    while visited.contains(&current.0) {
      current = queue.extract_minimum();
    }
  }

  queue.extract_minimum().1
}

fn solve_part_1(grid: Vec<Vec<u32>>) -> u32 {
  solve_with_djikstra::<HashmapQueue>(grid, 1, 3)
}

pub fn part_1() -> u32 {
  let grid = read_file();
  solve_part_1(grid)
}

fn solve_part_2(grid: Vec<Vec<u32>>) -> u32 {
  solve_with_djikstra::<HashmapQueue>(grid, 4, 10)
}

pub fn part_2() -> u32 {
  let grid = read_file();
  solve_part_2(grid)
}
