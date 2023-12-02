use std::fs::File;
use std::io::prelude::*;

struct CubeReveal {
  red: u32,
  green: u32,
  blue: u32
}

impl CubeReveal {
  fn new() -> Self {
    CubeReveal { red: 0, green: 0, blue: 0 }
  }
}

struct Game {
  id: u32,
  draws: Vec<CubeReveal>
}

fn parse_reveal(input: &str) -> CubeReveal {
  let mut result = CubeReveal::new();
  let reveal_parts = input.split(", ");
  for part in reveal_parts {
    let vec_parts: Vec<&str> = part.split(" ").collect();
    let number = vec_parts[0].parse().unwrap();
    let color = vec_parts[1];
    match color {
      "red" => result.red = number,
      "green" => result.green = number,
      "blue" => result.blue = number,
      s => panic!("unexpected color name {} revealed", s),
    }
  }
  result
}

fn read_line(line: &str) -> Game {
  let parts: Vec<&str> = line.split(": ").collect();
  let (game_id_part, reveal_parts) = parts.split_at(1);
  let id = game_id_part[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
  let draws = reveal_parts[0].split("; ").into_iter().map(|s| parse_reveal(s)).collect();
  Game {
    id,
    draws
  }
}

fn read_file() -> Vec<Game> {
  let mut file = File::open("./input/input2.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents.lines().map(read_line).collect()
}


fn solve_part_1(v: Vec<Game>) -> u32 {
  let mut id_sum = 0;
  for game in v {
    let mut game_is_ok = true;
    for draw in game.draws {
      if draw.red > 12 || draw.green > 13 || draw.blue > 14 {
        game_is_ok = false;
        break;
      }
    }
    if game_is_ok {
      id_sum += game.id;
    }
  }
  id_sum
}

pub fn part_1() -> u32 {
  let games = read_file();
  solve_part_1(games)
}

fn get_minimum_power(game: &Game) -> u32 {
  let min_red = game.draws.iter().map(|draw| draw.red).max().unwrap();
  let min_green = game.draws.iter().map(|draw| draw.green).max().unwrap();
  let min_blue = game.draws.iter().map(|draw| draw.blue).max().unwrap();
  min_red * min_green * min_blue
}

fn solve_part_2(v: Vec<Game>) -> u32 {
  v.iter().map(|game| get_minimum_power(game)).sum()
}

pub fn part_2() -> u32 {
  let games = read_file();
  solve_part_2(games)
}
