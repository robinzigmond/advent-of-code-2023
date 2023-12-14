use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
enum SpringCondition {
  Operational,
  Damaged,
  Unknown,
}

#[derive(Clone)]
struct Row {
  springs: Vec<SpringCondition>,
  groups: Vec<u64>,
}

fn read_line(line: &str) -> Row {
  let parts: Vec<&str> = line.split(" ").collect();
  let springs = parts[0];
  let groups = parts[1];

  let springs = springs.chars().map(|c| match c {
    '#' => SpringCondition::Damaged,
    '.' => SpringCondition::Operational,
    '?' => SpringCondition::Unknown,
    _ => panic!("unexpected spring character: {}", c),
  }).collect();
  let groups = groups.split(",").map(|digit| digit.parse().unwrap()).collect();

  Row { springs, groups }
}

fn read_file() -> Vec<Row> {
  let mut file = File::open("./input/input12.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  contents.lines().map(read_line).collect()
}

// I first tried this puzzle with a brute-force approach - essentially trying all possible ways of filling in
// the unknown springs (only short-circuiting in really obvious cases), and counting how many fit the pattern.
// This actually worked fine for part 1 but runs for an age for part 2 - so a new approach was needed.

// The new approach is explained later in these comments - but in order to make it easier to figure out
// (well I think it did but I can't be totally sure - I worked on this for a few days, and needed some mild
// help, before I got it all working in a reasonable runtime!) - I changed the way the spring patterns are
// encoded.
// This is essentially a RLE (= run-length-encoding) of the data - noting first that that any number of .'s
// (operational springs) is equivalent to just 1 when it comes to counting the number of solutions.
// So we can just keep a vector of "pieces", each of which are just sequences of #s and ?s.
// These sequences are what the RLE form encodes.
// To simplify things a little further, the RLE form will is be a sequence (vec) of ints, assumed to
// always start on "unknown", which will have a leading 0 if it instead starts on a known damaged spring.

fn rle_encode(springs: &Vec<SpringCondition>) -> Vec<Vec<u64>> {
  let mut result = vec![];

  let mut last_piece_known = false;
  let mut number_so_far = 0;
  let mut current_rle_piece = vec![];
  for spring in springs {
    match spring {
      SpringCondition::Damaged => {
        if last_piece_known {
          number_so_far += 1;
        } else {
          current_rle_piece.push(number_so_far);
          number_so_far = 1;
        }
        last_piece_known = true;
      },
      SpringCondition::Unknown => {
        if last_piece_known {
          current_rle_piece.push(number_so_far);
          number_so_far = 1;
        } else {
          number_so_far += 1;
        }
        last_piece_known = false;
      },
      SpringCondition::Operational => {
        if number_so_far > 0 {
          current_rle_piece.push(number_so_far);
          result.push(current_rle_piece);
          current_rle_piece = vec![];
          last_piece_known = false;
          number_so_far = 0;
        }
      },
    }
  }

  if number_so_far > 0 {
    current_rle_piece.push(number_so_far);
    result.push(current_rle_piece);
  }

  result
}

// This is one of 2 recursive functions that the solution relies on.
// It takes a single run-length-encoded set of springs, and a list of groups to make, and works out how many
// different ways there are of fitting that set of groups into the set of springs.
// In other words, this solves the puzzle in the special case that there are no `.`s in the input string (after trimming
// `.`s from either end).
// IMPORTANT NOTE: the third paramater is a hashmap which memoises the results of this function. This turns out to be vital
// in order to solve part 2 in any reasonable time, as there turn out to be A LOT of calls to this function with the same
// data (vastly more than I initially thought possible).
fn count_fits(rle_piece: &Vec<u64>, groups: &Vec<u64>, previous_results: &mut HashMap<(Vec<u64>, Vec<u64>), u64>) -> u64 {
  // use memoised result, if it exists
  if let Some(&ans) = previous_results.get(&(rle_piece.clone(), groups.clone())) {
    return ans;
  }

  // the base case is when we're out of groups. There's nothing more to do, so we can only have already succeeded
  // or failed. If we have any `#`s left (ie damaged springs) then we've failed, as these will give an extra group
  // that we don't have - but if not, we've succeeded.
  if groups.is_empty() {
    let solution = if rle_piece.len() > 1 { 0 } else { 1 };
    previous_results.insert((rle_piece.clone(), groups.clone()), solution);
    return solution;
  }

  // if we still have groups left but are out of places to fit them, there must be no solution
  if rle_piece.is_empty() {
    previous_results.insert((rle_piece.clone(), groups.clone()), 0);
    return 0;
  }

  let (next_group, rest) = groups.split_at(1);
  let next_group = next_group[0];
  let rest = rest.to_vec();

  // we now work out at which possible indexes we can start fitting in the `#`s. We only care about the first
  // group for now - if some positions leave not enough space to fit later groups in what remains, we discover that
  // in one of the recursive calls below.
  let piece_length: u64 = rle_piece.iter().sum();
  let mut total_fits = 0;
  let mut possible_starts = vec![];

  // First note that when we're in the second "piece" of the RLE, that corresponds to a known spring - we can start
  // the group at the first of these (if the ? group before it is all filled in with .'s), but not at any point after.
  // This determines the upper limit on the below loop.
  for possible_index in 0..=rle_piece[0] {
    let mut is_still_possible = true;
    // The index computed here is the one that corresponds to the next spring immediately after the group has
    // finished - it's "forbidden" to be damaged, because if it is it will increase the size of the group and
    // therefore it won't work.
    let forbidden_index = possible_index + next_group;
    if forbidden_index > piece_length {
      // This means that there aren't enough springs in the piece to fit the group when we start here.
      // So not only is this index not possible, nor are any later ones - so we can exit.
      break;
    } else {
      // we need to loop through the RLE-representation to determine whether the "forbidden" spring is damaged
      // or unknown
      let mut in_unknown_section = true;
      let mut length_so_far = 0;
      for section in rle_piece {
        length_so_far += section;
        if length_so_far > forbidden_index {
          // if we get here, this is the section containing the "forbidden" spring, so:
          if !in_unknown_section {
            is_still_possible = false;
          }
          break;
        }
        in_unknown_section = !in_unknown_section;
      }
    }
    if is_still_possible {
      possible_starts.push(possible_index);
    }
  }

  for start in possible_starts {
    // call this function recursively, to see how many times we can fit the remaining groups
    // into what we have left.
    // The RLE of what we have left will be the old RLE but with (start + 1) units removed from
    // the start (+ 1 because there must be at least one Operational spring - a `.` - after the first group
    // - in our earlier checks we guaranteed that this position has an unknown spring.
    // That also means we don't have to carefully track parity in the RLE as we did above -
    // as the loop below will naturally start from an "unknown" section, possibly of length 0, as required).
    let mut remaining_rle = vec![];
    let mut found_start = false;
    let mut length_so_far = 0;
    let mut one_carried_over = false;
    for &section in rle_piece {
      length_so_far += section;
      if found_start {
        // if we've already found the end of the group, and therefore where the new RLE starts,
        // we just copy the remaining pieces over. The exception is if we've only just found the
        // end and it was right at the end of a section of #s, when there must be a . afterward
        // which reduces the length of the ? section that follows by 1. This is what `one_carried_over`
        // denotes (see comment below).
        let mut new_group = section;
        if one_carried_over {
          new_group -= 1;
          one_carried_over = false;
        }
        remaining_rle.push(new_group);
      } else if length_so_far > start + next_group {
        found_start = true;
        let mut remaining_group = length_so_far - start - next_group;
        // We have to remove an additional one from what remains of the piece, to account for the .
        // which must follow. But if we're already at the end of the piece, we can't do this and
        // must instead remove it from the start of the next piece - which is what the `one_carried_over`
        // flag is for (see comment above).
        if remaining_group == 0 {
          one_carried_over = true;
        } else {
          remaining_group -= 1;
        }
        remaining_rle.push(remaining_group);
      }
    }
    total_fits += count_fits(&remaining_rle, &rest, previous_results);
  }

  // memoise result
  previous_results.insert((rle_piece.clone(), groups.clone()), total_fits);
  total_fits
}

// this is the "main" recursive function, which solves the more general case where there are several pieces separated by one
// or more `.`s.
fn solve(rle_pieces: &Vec<Vec<u64>>, groups: &Vec<u64>, previous_results: &mut HashMap<(Vec<u64>, Vec<u64>), u64>) -> u64 {
  // base case: if we've run out of pieces, there's nothing left to do, including no choice about it
  // That means a single solution if we've accounted for all groups of damaged springs by now, otherwise no solution.
  if rle_pieces.is_empty() {
    return if groups.is_empty() { 1 } else { 0 };
  }

  // start counting the number of solutions
  let mut num_solutions = 0;

  // pull out the next piece to try to fit groups into - and keep the rest for later
  let (next_piece, rest) = rle_pieces.split_at(1);
  let next_piece = &next_piece[0];
  let rest = rest.to_vec();

  // work out what is the maximum number of remaining groups that can be fit inside the next piece
  let piece_length = next_piece.iter().sum();
  let mut length_needed = 0;
  let mut max_number_of_groups = 0;
  for group in groups {
    length_needed += group;
    if length_needed > piece_length {
      break;
    }
    max_number_of_groups += 1;
    // we need an additional 1 added to the length to accomodate the "break" (operational pipe) betweem the groups
    // if we want to fit another group
    length_needed += 1;
  }

  for index in 0..max_number_of_groups {
    let (groups_to_fit, remaining_groups) = groups.split_at(index + 1);
    let groups_to_fit = groups_to_fit.to_vec();
    let remaining_groups = remaining_groups.to_vec();
    // work out the number of different ways of fitting those pieces inside the first group, using the function above
    let number_of_fits = count_fits(next_piece, &groups_to_fit, previous_results);
    // don't waste time recursing if we already know there's no solutions!
    if number_of_fits > 0 {
      num_solutions += number_of_fits * solve(&rest, &remaining_groups, previous_results);
    }
  }

  // There is an annoying special case to take account of here!
  // It can happen that the first piece is too small for the first group - a simple example would be ?..## with a
  // single group of 2. That certainly has a solution (and only one), but the above loop won't even run because there
  // is no way to fit 2 springs into the first ? (which is essentially irrelevant).
  // In these cases we need to completely ignore that first piece.
  // Luckily we can easily tell this from our RLE representation - it happens when the first piece if all ?'s, which
  // is when the RLE has only a single element.
  if next_piece.len() == 1 {
    num_solutions += solve(&rest, groups, previous_results);
  }

  num_solutions
}

fn get_number_of_combinations(row: &Row, previous_results: &mut HashMap<(Vec<u64>, Vec<u64>), u64>) -> u64 {
  let rle_encoding = rle_encode(&row.springs);
  solve(&rle_encoding, &row.groups, previous_results)
}

fn solve_part_1(rows: &Vec<Row>) -> u64 {
  let mut fit_results = HashMap::new();
  rows.iter().map(|row| get_number_of_combinations(row, &mut fit_results)).sum()
}

pub fn part_1() -> u64 {
  let rows = read_file();
  solve_part_1(&rows)
}

// unfolds the row to transform from part 1 to part 2
fn unfold(row: &Row) -> Row {
  let Row { springs, groups } = row;
  let mut unfolded_springs = vec![];
  let mut unfolded_groups = vec![];

  for i in 0..5 {
    unfolded_springs.append(&mut springs.clone());
    unfolded_groups.append(&mut groups.clone());
    if i < 4 {
      unfolded_springs.push(SpringCondition::Unknown);
    }
  }

  Row { springs: unfolded_springs, groups: unfolded_groups }
}

fn solve_part_2(rows: &Vec<Row>) -> u64 {
  let mut fit_results = HashMap::new();
  rows.iter().map(|row| get_number_of_combinations(&unfold(row), &mut fit_results)).sum()
}

pub fn part_2() -> u64 {
  let rows = read_file();
  solve_part_2(&rows)
}