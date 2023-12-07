use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
  Blank(u8), // we need these fictional "blank" cards to make things easier in solving the second part
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}

#[derive(Clone)]
struct Hand {
  cards: [Card; 5],
  bid: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

fn read_line(l: &str) -> Hand {
  let parts: Vec<&str> = l.split(" ").collect();
  let bid = parts[1].parse().unwrap();
  let cards: Vec<Card> = parts[0].chars().map(|c| match c {
    '2' => Card::Two,
    '3' => Card::Three,
    '4' => Card::Four,
    '5' => Card::Five,
    '6' => Card::Six,
    '7' => Card::Seven,
    '8' => Card::Eight,
    '9' => Card::Nine,
    'T' => Card::Ten,
    'J' => Card::Jack,
    'Q' => Card::Queen,
    'K' => Card::King,
    'A' => Card::Ace,
    other => panic!("unexpected character for card: {}", other),
  }).collect();

  let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];

  Hand { cards, bid }
}

fn read_file() -> Vec<Hand> {
  let mut file = File::open("./input/input7.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  contents.lines().map(read_line).collect()
}

fn get_type(hand: &Hand) -> HandType {
  let mut set_of_cards = HashSet::new();
  for card in hand.cards {
    set_of_cards.insert(card);
  }
  match set_of_cards.len() {
    1 => return HandType::FiveOfAKind,
    2 => {
      // determine if it's a 4/1 split or a 3/2
      for card in &set_of_cards {
        // we're not actually iterating, as we'll return on the first iteration.
        // This is just a way to get hold of a single card from the set!
        return match hand.cards.iter().filter(|&c| c == card).count() {
          1 | 4 => HandType::FourOfAKind,
          2 | 3 => HandType::FullHouse,
          n => panic!("two different ranks but {} occurences of one of them?!", n),
        } 
      }
      panic!("no cards yet 2 different types - impossible");
    }
    3 => {
      // same as above, to determine if it's two pair of three of a kind
      // determine if it's a 4/1 split or a 3/2. This time though we might
      // have to iterate through more ranks in the set.
      for card in &set_of_cards {
        // we're not actually iterating, as we'll return on the first iteration.
        // This is just a way to get hold of a single card from the set!
        match hand.cards.iter().filter(|&c| c == card).count() {
          3 => return HandType::ThreeOfAKind,
          2 => return HandType::TwoPair,
          1 => (), // this doesn't tell us anything, so go to next card
          n => panic!("three different ranks but {} occurences of one of them?!", n),
        } 
      }
      panic!("no cards yet 3 different types - impossible");
    }
    4 => return HandType::OnePair,
    5 => return HandType::HighCard,
    _ => panic!("more than 5 cards??"),
  }
}

fn solve_part_1(hands: &mut Vec<Hand>) -> u32 {
  hands.sort_by(|a, b| match get_type(a).cmp(&get_type(b)) {
    Ordering::Equal => a.cards.cmp(&b.cards),
    other => other,
  });
  hands.iter().enumerate().map(|(index, hand)| (index as u32 + 1) * hand.bid).sum()
}

pub fn part_1() -> u32 {
  let mut hands = read_file();
  solve_part_1(&mut hands)
}

// we need this utility in a couple of different places - to remove jacks/jokers and consider a hand
// where they are replaced with low-ranking, distinct, "blank" cards
fn replace_jokers_with_blanks(hand: &Hand) -> Hand {
  let mut hand_copy = hand.clone();
  let mut jokers_found = 0;
  for card in hand_copy.cards.iter_mut() {
    if card == &Card::Jack {
      *card = Card::Blank(jokers_found);
      jokers_found += 1;
    }
  }
  hand_copy
}

fn get_type_with_jokers(hand: &Hand) -> HandType {
  let joker_count = hand.cards.iter().filter(|&c| c == &Card::Jack).count();
  match joker_count {
    0 => return get_type(hand),
    1 => {
      // first see what the hand type would be with the joker replaced by a unique "blank" card
      let with_blanks = replace_jokers_with_blanks(hand);
      // now we can easily determine the best hand we can get from replacing the joker with another card of our choice
      let old_type = get_type(&with_blanks);
      return match old_type {
        HandType::FiveOfAKind => panic!("can't have 5 of a kind in a hand with only one jack/joker!"),
        HandType::FourOfAKind => HandType::FiveOfAKind,
        HandType::FullHouse => panic!("can't have a full house in a hand with only one jack/joker!"),
        HandType::ThreeOfAKind => HandType::FourOfAKind,
        HandType::TwoPair => HandType::FullHouse,
        HandType::OnePair => HandType::ThreeOfAKind,
        HandType::HighCard => HandType::OnePair,
      }
    }
    2 => {
      // very similar to before
      let with_blanks = replace_jokers_with_blanks(hand);
      // now we can easily determine the best hand we can get from replacing the joker with another card of our choice
      let old_type = get_type(&with_blanks);
      return match old_type {
        HandType::FiveOfAKind => panic!("can't have 5 of a kind in a hand with 2 distinct jacks/jokers!"),
        HandType::FourOfAKind => panic!("can't have 4 of a kind in a hand with 2 distinct jacks/jokers!"),
        HandType::FullHouse => panic!("can't have a full house in a hand with 2 distinct jacks/jokers!"),
        HandType::ThreeOfAKind => HandType::FiveOfAKind,
        HandType::TwoPair => panic!("can't have 2 pair in a hand with 2 distinct jacks/jokers!"),
        HandType::OnePair => HandType::FourOfAKind,
        HandType::HighCard => HandType::ThreeOfAKind,
      }
    }
    3 => {
      let with_blanks = replace_jokers_with_blanks(hand);
      // now we can easily determine the best hand we can get from replacing the joker with another card of our choice
      let old_type = get_type(&with_blanks);
      return match old_type {
        HandType::FiveOfAKind => panic!("can't have 5 of a kind in a hand with 3 distinct jacks/jokers!"),
        HandType::FourOfAKind => panic!("can't have 4 of a kind in a hand with 3 distinct jacks/jokers!"),
        HandType::FullHouse => panic!("can't have a full house in a hand with 3 distinct jacks/jokers!"),
        HandType::ThreeOfAKind => panic!("can't have a 3 of a kind in a hand with 3 distinct jacks/jokers!"),
        HandType::TwoPair => panic!("can't have 2 pair in a hand with 3 distinct jacks/jokers!"),
        HandType::OnePair => HandType::FiveOfAKind,
        HandType::HighCard => HandType::FourOfAKind,
      }
    }
    4 | 5 => HandType::FiveOfAKind, // trivial to always make 5 of a kind if 4 or all 5 of the 5 cards are wild!
    count => panic!("we somehow have exactly {} jokers...", count),
  }
}

fn solve_part_2(hands: &mut Vec<Hand>) -> u32 {
  hands.sort_by(|a, b| match get_type_with_jokers(a).cmp(&get_type_with_jokers(b)) {
    Ordering::Equal => replace_jokers_with_blanks(a).cards.cmp(&replace_jokers_with_blanks(b).cards),
    other => other,
  });
  hands.iter().enumerate().map(|(index, hand)| (index as u32 + 1) * hand.bid).sum()
}

pub fn part_2() -> u32 {
  let mut hands = read_file();
  solve_part_2(&mut hands)
}
