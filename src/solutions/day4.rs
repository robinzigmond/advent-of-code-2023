use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

struct Card {
  winning: HashSet<u32>,
  actual: HashSet<u32>,
}

fn read_line(line: &str) -> Card {
  let parts: Vec<&str> = line.split(" | ").collect();
  let winning_str = parts[0].split(": ").collect::<Vec<&str>>()[1];
  let actual_str = parts[1];
  let mut winning = HashSet::new();
  let mut actual = HashSet::new();
  for num_part in winning_str.split_whitespace() {
    winning.insert(num_part.parse().unwrap());
  }
  for num_part in actual_str.split_whitespace() {
    actual.insert(num_part.parse().unwrap());
  }
  
  Card {
    winning,
    actual,
  }
}

fn read_file() -> Vec<Card> {
  let mut file = File::open("./input/input4.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  
  contents.lines().map(read_line).collect()
}

fn solve_part_1(cards: Vec<Card>) -> u32 {
  cards.iter().map(|card| {
    let number_of_winners = card.winning.intersection(&card.actual).count();
    if number_of_winners == 0 {
      return 0;
    }
    return 2u32.pow(number_of_winners as u32 - 1)
  }).sum() 
}

pub fn part_1() -> u32 {
  let cards = read_file();
  solve_part_1(cards)
}

fn solve_part_2(cards: Vec<Card>) -> u32 {
  // keep track of how many copies of each card we have
  let mut card_copies: Vec<u32> = cards.iter().map(|_| 1).collect();
  for (index, card) in cards.iter().enumerate() {
    let number_of_winners = card.winning.intersection(&card.actual).count();
    let copies_of_current_card = card_copies[index];
    for i in 1..=number_of_winners {
      let new_index = index + i;
      if new_index < card_copies.len() {
        card_copies[new_index] += copies_of_current_card;
      }
    }
  }
  card_copies.iter().sum()
}

pub fn part_2() -> u32 {
  let cards = read_file();
  solve_part_2(cards)
}
