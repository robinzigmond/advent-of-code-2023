use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

enum Direction {
  North,
  South,
  East,
  West,
}

enum Space {
  Start,
  End,
  Path,
  Forest,
  Slope(Direction),
}

fn read_file() -> Vec<Vec<Space>> {
  let mut file = File::open("./input/input23.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut map = vec![];
  let num_lines = contents.lines().count();

  for (row_no, row) in contents.lines().enumerate() {
    let mut line = vec![];
    for c in row.chars() {
      let space = match c {
        '.' => if row_no == 0 {
            Space::Start
          } else if row_no == num_lines - 1 {
            Space::End
          } else {
            Space::Path
          },
        '#' => Space::Forest,
        '^' => Space::Slope(Direction::North),
        'v' => Space::Slope(Direction::South),
        '>' => Space::Slope(Direction::East),
        '<' => Space::Slope(Direction::West),
        _ => panic!("unexpected map character: {}", c),
      };
      line.push(space);
    }

    map.push(line);
  }

  map
}

// going to transform the data to a simpler form first

struct GraphNode {
  connections: Vec<(usize, usize, u32)>,
}

fn get_neighbours(map: &Vec<Vec<Space>>, row: usize, col: usize) -> Vec<(usize, usize)> {
  let mut neighbours = vec![];
  
  if row > 0 {
    neighbours.push((row - 1, col));
  }
  if row < map.len() - 1 {
    neighbours.push((row + 1, col));
  }
  if col > 0 {
    neighbours.push((row, col - 1));
  }
  if col < map[0].len() - 1 {
    neighbours.push((row, col + 1));
  }

  neighbours.iter().filter(|(i, j)| match map[*i][*j] {
    Space::Forest => false,
    _ => true,
  }).map(|coords| coords.to_owned()).collect()
}

fn get_connections(map: &Vec<Vec<Space>>, row: usize, col: usize) -> Vec<(usize, usize, u32)> {
  let mut connections = vec![];

  let neighbours = get_neighbours(map, row, col);

  for neighbour in neighbours {
    let mut current_point = Some(neighbour);
    let mut previous = (row, col);
    // need to exclude the point we've just come from
    let mut current_neighbours: Vec<(usize, usize)> = get_neighbours(map, neighbour.0, neighbour.1)
      .iter()
      .map(|n| n.to_owned())
      .filter(|&n| n != previous)
      .collect();
    let mut distance = 1;
    while current_neighbours.len() == 1 {
      distance += 1;
      previous = current_point.unwrap();
      current_point = Some(current_neighbours[0]);
      // ensure we follow any slopes
      if let Space::Slope(dir) = &map[current_neighbours[0].0][current_neighbours[0].1] {
        distance += 1;
        match dir {
          Direction::North => {
            current_point = Some((current_neighbours[0].0 - 1, current_neighbours[0].1));
          },
          Direction::South => {
            current_point = Some((current_neighbours[0].0 + 1, current_neighbours[0].1));
          },
          Direction::East => {
            current_point = Some((current_neighbours[0].0, current_neighbours[0].1 + 1));
          },
          Direction::West => {
            current_point = Some((current_neighbours[0].0, current_neighbours[0].1 - 1));
          },
        }
      }
      // abandon this passage if we've been pushed back the way we came (which can happen with the slopes)
      if current_point == Some(previous) {
        current_point = None;
        break;
      }
      // need to exclude the point we've just come from
      current_neighbours = get_neighbours(map, current_point.unwrap().0, current_point.unwrap().1)
        .iter()
        .map(|n| n.to_owned())
        .filter(|&n| n != previous)
        .collect();
    }
    if current_point.is_some() {
      let point = current_point.unwrap();
      connections.push((point.0, point.1, distance));
    }
  }

  connections
}

fn transform_data(map: &Vec<Vec<Space>>) -> HashMap<(usize, usize), GraphNode> {
  let mut graph = HashMap::new();

  for (row_index, row) in map.iter().enumerate() {
    for (col_index, space) in row.iter().enumerate() {
      match space {
        // it's only a genuine node if it's either the start/end node or it has at least 3
        // neighbours
        Space::Start | Space::End => {
          let connections = get_connections(map, row_index, col_index);
          graph.insert((row_index, col_index), GraphNode { connections });
        },
        Space::Path | Space::Slope(_) => {
          let num_neighbours = get_neighbours(map, row_index, col_index).len();
          if num_neighbours > 2 {
            let connections = get_connections(map, row_index, col_index);
            graph.insert((row_index, col_index), GraphNode { connections });
          }
        },
        _ => (),
      }
    }
  }

  graph
}

// recursive function that does the main work for finding the longest route on the graph representation
fn find_all_routes(graph: &HashMap<(usize, usize), GraphNode>, start_row: usize, start_col: usize, current_route: Vec<(usize, usize)>) -> Vec<u32> {
  // abandon if we've already been here!
  if current_route.contains(&(start_row, start_col)) {
    return vec![];
  }
  
  let max_row = graph.keys().map(|(i, _)| i).max().unwrap();
  let mut distances_found = vec![];

  for (row, col, distance) in &graph.get(&(start_row, start_col)).unwrap().connections {
    if row == max_row {
      // we've found the end, so just add the distance
      distances_found.push(*distance);
    } else {
      // recursive find all routes from here, and add on the distance from the current start
      let mut new_route = current_route.clone();
      new_route.push((start_row, start_col));
      let routes_from_here = find_all_routes(graph, *row, *col, new_route);
      for old_distance in routes_from_here {
        distances_found.push(old_distance + distance);
      }
    }
  }

  distances_found
}

fn solve_part_1(graph: &HashMap<(usize, usize), GraphNode>) -> u32 {
  let (start_row, start_col) = graph.keys().find(|(i, _)| *i == 0).unwrap();
  *find_all_routes(graph, *start_row, *start_col, vec![]).iter().max().unwrap()
}

pub fn part_1() -> u32 {
  let map = read_file();
  let graph = transform_data(&map);
  solve_part_1(&graph)
}

pub fn part_2() -> u32 {
  let mut map = read_file();
  // need to replace all slopes with plain path:
  for row in &mut map {
    for space in row {
      match space {
        Space::Slope(_) => *space = Space::Path,
        _ => (),
      }
    }
  }
  let graph = transform_data(&map);
  solve_part_1(&graph)
}
