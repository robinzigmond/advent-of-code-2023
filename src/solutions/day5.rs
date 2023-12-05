use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
struct MapLine {
  destination_start: u64,
  source_start: u64,
  range_length: u64,
}

#[derive(Clone)]
struct Almanac {
  seeds: Vec<u64>,
  maps: Vec<Vec<MapLine>>,
}

fn read_file() -> Almanac {
  let mut file = File::open("./input/input5.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  
  let all_lines: Vec<&str> = contents.lines().collect();
  let first_line_space_separated: Vec<String> = all_lines[0].split(" ").map(|s| s.to_owned()).collect();
  let seeds = first_line_space_separated.split_at(1).1.to_owned().iter().map(|s| s.parse().unwrap()).collect();

  let mut maps = vec![];
  let mut current_map = vec![];
  for &line in all_lines.split_at(2).1 {
    if line.is_empty() {
      // test for blank line (should just contain newline character(s))
      maps.push(current_map.clone());
      current_map.clear();
    } else if line.starts_with(|c: char| c.is_digit(10)) {
      // ignore the "title" lines which don't start with a number
      let nums: Vec<u64> = line.split(" ").map(|s| s.parse().unwrap()).collect();
      current_map.push(MapLine { destination_start: nums[0], source_start: nums[1], range_length: nums[2] });
    }
  }
  // the above won't handle the final map properly - there is no blank line at the end of the file - so we do it here
  maps.push(current_map);

  Almanac { seeds, maps }
}

fn follow_map_line(map_line: MapLine, num: u64) -> u64 {
  let MapLine { source_start, destination_start, range_length } = map_line;
  if num >= source_start && num < source_start + range_length {
    destination_start + num - source_start
  } else {
    num
  }
}

fn get_seed_destination(seed: u64, maps: Vec<Vec<MapLine>>) -> u64 {
  let mut current = seed;
  for map in maps {
    for line in map {
      let result = follow_map_line(line, current);
      if result != current {
        current = result;
        break;
      }
    }
  }
  current
}

fn solve_part_1(almanac: Almanac) -> u64 {
  let Almanac { seeds, maps } = almanac;
  seeds.iter().map(|&seed| get_seed_destination(seed, maps.clone())).min().unwrap()
}

pub fn part_1() -> u64 {
  let almanac = read_file();
  solve_part_1(almanac)
}

// idea for part 2 - clearly it isn't practically to run through the full algorithm on all billions of inputs listed.
// But the mappings appear to be laid out in a one-to-one way. Meaning that we can start with 1 as a desired output, easily
// work through which input seed would be needed to get that, and check if it's in our starting data. If not (of course it
// won't be!), try 2 and keep going.
// Although this will probably take tens/hundreds of millions of tries, that's a lot fewer than the "obvious" brute force
// approach.
// [turns out to be around 25 seconds when compiled in release mode. Not great for day 5 - but good enough!]

fn follow_map_line_backwards(map_line: MapLine, num: u64) -> u64 {
  let MapLine { source_start, destination_start, range_length } = map_line;
  if num >= destination_start && num < destination_start + range_length {
    source_start + num - destination_start
  } else {
    num
  }
}

fn get_initial_seed_for_location(seed: u64, maps: &Vec<Vec<MapLine>>) -> u64 {
  // need to reverse the order we traverse the maps in
  let mut reversed_maps = maps.clone();
  reversed_maps.reverse();
  let mut current = seed;
  for map in reversed_maps {
    for line in map {
      let result = follow_map_line_backwards(line, current);
      if result != current {
        current = result;
        break;
      }
    }
  }
  current
}

fn solve_part_2(almanac: Almanac) -> u64 {
  let Almanac { seeds, maps } = almanac;
  // first split the "seeds" into start and end ranges
  let mut ranges = vec![];
  for (index, &number) in seeds.iter().enumerate() {
    if index % 2 == 1 {
      ranges.push((seeds[index - 1], number));
    }
  } 
  let mut minimum = 1;
  loop {
    let starting_seed = get_initial_seed_for_location(minimum, &maps);
    for &(start, range) in &ranges {
      if start <= starting_seed && starting_seed < start + range {
        return minimum;
      }
    }
    minimum += 1;
  }
}

pub fn part_2() -> u64 {
  let almanac = read_file();
  solve_part_2(almanac)
}
