use std::fs::File;
use std::io::prelude::*;

struct Race {
  time: u64,
  distance: u64,
}

fn read_file() -> Vec<Race> {
  let mut file = File::open("./input/input6.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut races = vec![];
  let mut times = vec![];
  let mut distances = vec![];

  let all_lines: Vec<&str> = contents.lines().collect();
  let times_line = all_lines[0];
  let distances_line = all_lines[1];

  for time_number in times_line.split_whitespace().skip(1) {
    times.push(time_number.parse().unwrap());
  }

  for distance_number in distances_line.split_whitespace().skip(1) {
    distances.push(distance_number.parse().unwrap());
  }

  for (index, &time) in times.iter().enumerate() {
    races.push(Race { time, distance: distances[index] });
  }

  races
}

fn ways_to_win(race: &Race) -> u64 {
  for n in 1..race.time {
    let distance = n * (race.time - n);
    if distance > race.distance {
      // we can exit now without continuing the loop, since
      // n * (m-n) peaks at n = m/2 and is symmetric either side of that.
      return race.time - 2 * (n - 1) - 1;
    }
  }
  panic!("not possible to win the race!");
}

fn solve_part_1(races: Vec<Race>) -> u64 {
  races.iter().map(|r| ways_to_win(r)).product()
}

pub fn part_1() -> u64 {
  let races = read_file();
  solve_part_1(races)
}

fn solve_part_2(races: Vec<Race>) -> u64 {
  let real_time = races.iter().map(|r| r.time.to_string()).collect::<String>().parse().unwrap();
  let real_distance = races.iter().map(|r| r.distance.to_string()).collect::<String>().parse().unwrap();
  ways_to_win(&Race { time: real_time, distance: real_distance })
}

pub fn part_2() -> u64 {
  let races = read_file();
  solve_part_2(races)
}
