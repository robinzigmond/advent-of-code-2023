use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct Part {
  x: u64,
  m: u64,
  a: u64,
  s: u64,
}

#[derive(Clone)]
enum PartDestination {
  Rejected,
  Accepted,
  Rule(String),
}

// attaching a function to a struct in Rust is quite awkward to get the compiler to accept,
// so rather than doing that I'm just using the below "statically encoded" form of the simple
// function/test
#[derive(Clone)]
struct Test {
  test_property: char,
  test_operation: char,
  test_comparison: u64,
}

#[derive(Clone)]
struct Rule {
  test: Option<Test>,
  destination: PartDestination,
}

struct PuzzleData {
  parts: Vec<Part>,
  rules: HashMap<String, Vec<Rule>>,
}

fn parse_part(line: &str) -> Part {
  let relevant: String = line.chars().filter(|c| !['{', '}'].contains(c)).collect();
  let parts = relevant.split(",");
  // initial values won't be needed, but the compiler (understandably) complains without them!
  let mut x = 0;
  let mut m = 0;
  let mut a = 0;
  let mut s = 0;
  for part in parts {
    let sides: Vec<&str> = part.split("=").collect();
    match sides[0] {
      "x" => x = sides[1].parse().unwrap(),
      "m" => m = sides[1].parse().unwrap(),
      "a" => a = sides[1].parse().unwrap(),
      "s" => s = sides[1].parse().unwrap(),
      s => panic!("unexpected part rating: {}", s),
    }
  }

  Part { x, m, a, s}
}

fn parse_rule(text: &str) -> Rule {
  if text == "R" {
    return Rule { test: None, destination: PartDestination::Rejected };
  }

  if text == "A" {
    return Rule { test: None, destination: PartDestination::Accepted };
  }

  let sides: Vec<&str> = text.split(":").collect();

  if sides.len() == 1 {
    return Rule { test: None, destination: PartDestination::Rule(sides[0].to_owned()) };
  }


  let destination = match sides[1] {
    "R" => PartDestination::Rejected,
    "A" => PartDestination::Accepted,
    other => PartDestination::Rule(other.to_owned()),
  };

  let test_vec: Vec<char> = sides[0].chars().collect();
  let test_property = test_vec[0];
  let test_operation = test_vec[1];
  let test_comparison: u64 = test_vec.split_at(2).1.iter().collect::<String>().parse().unwrap();

  let test = Some(Test { test_property, test_operation, test_comparison });

  Rule { destination, test }
}

fn read_file() -> PuzzleData {
  let mut file = File::open("./input/input19.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut parts = vec![];
  let mut rules = HashMap::new();
  let mut finished_rules = false;

  for line in contents.lines() {
    if line.is_empty() {
      finished_rules = true;
      continue;
    }

    if finished_rules {
      parts.push(parse_part(line));
    } else {
      let parts: Vec<&str> = line.split(['{', '}']).collect();
      let label = parts[0].to_owned();
      rules.insert(label, parts[1].split(",").map(parse_rule).collect());
    }
  }

  PuzzleData { parts, rules }
}

fn apply_test(part: &Part, test: &Test) -> bool {
  let Test { test_property, test_operation, test_comparison } = test;
  let value_to_test = match test_property {
    'x' => part.x,
    'm' => part.m,
    'a' => part.a,
    's' => part.s,
    _ => panic!("unexpected test property: {}", test_property),
  };
  match test_operation {
    '>' => value_to_test > *test_comparison,
    '<' => value_to_test < *test_comparison,
    _ => panic!("unexpected test operation: {}", test_operation),
  }
}

fn apply_ruleset(part: &Part, rules: &Vec<Rule>) -> PartDestination {
  for rule in rules {
    let Rule { test, destination } = rule;
    match test {
      Some(test) => {
        if apply_test(part, &test) {
          return destination.to_owned();
        }
      },
      None => {
        return destination.to_owned();
      },
    }
  }
  panic!("no destination found after all rules applied!");
}

fn is_part_accepted(part: &Part, rules: &HashMap<String, Vec<Rule>>) -> bool {
  let mut destination = String::from("in");

  loop {
    let ruleset = rules.get(&destination).unwrap();
    match apply_ruleset(part, ruleset) {
      PartDestination::Accepted => return true,
      PartDestination::Rejected => return false,
      PartDestination::Rule(label) => destination = label,
    }
  }
}

fn solve_part_1(data: &PuzzleData) -> u64 {
  let PuzzleData { parts, rules } = data;

  parts.iter().filter(|part| is_part_accepted(part, rules)).map(|part| part.x + part.m + part.a + part.s).sum()
}

pub fn part_1() -> u64 {
  let data = read_file();
  solve_part_1(&data)
}

// for part 2, it should simply be a cause of starting from "in", following each path we can go down,
// and keeping track of the region of 4-dimensional space (ie the valid values of the 4 paramaters)
// that we must be in in order to get to each destination. All paths end with R or A, so we just need
// to keep track of all the volumes that end with R, work out each one's 4-dimensional volume (easy
// as the boundaries will be a (hyper-)cuboid), and add these up (as by construction they must be
// pairwise distinct)

// we start with a type to represent the region
#[derive(Clone)]
struct FourDimensionalRegion {
  x_min: u64,
  x_max: u64,
  m_min: u64,
  m_max: u64,
  a_min: u64,
  a_max: u64,
  s_min: u64,
  s_max: u64,
}

// and some simple functions on it that we will need
impl FourDimensionalRegion {
  fn new() -> Self {
    Self {
      x_min: 1,
      x_max: 4000,
      m_min: 1,
      m_max: 4000,
      a_min: 1,
      a_max: 4000,
      s_min: 1,
      s_max: 4000,
    }
  }

  fn volume(&self) -> u64 {
    // note that we need these +1s, as these are by definition "inclusive" boundaries
    (self.x_max - self.x_min + 1)
    * (self.m_max - self.m_min + 1)
    * (self.a_max - self.a_min + 1)
    * (self.s_max - self.s_min + 1)
  }

  // it's important to have a test for emptiness, for 2 reasons.
  // One is that once we get an empty region we can stop on a particular path, which could in theory cut out
  // quite a lot of un-needed calculations.
  // The second is that if we represent it with say x_max = x_min, and the same for every other co-ordinate, the
  // above volume formula will still give an answer of 1 due to the +1s (and it may give a negative answer if
  // the min's end up well above the max's).
  // [Note: actualy x_max = x_min is NOT empty, it just means there's only one valid value of x.]
  // So what we'll do is use the naive intersection calculations (below) but then do the following test for
  // emptiness on the result.
  fn is_empty(&self) -> bool {
    (self.x_min > self.x_max) || (self.m_min > self.m_max) || (self.a_min > self.a_max) || (self.s_min > self.s_max)
  }

  fn reduce_for_test(&self, test: &Test) -> Self {
    let mut result = self.to_owned();
    let Test { test_property, test_operation, test_comparison } = test;
    match test_operation {
      '>' => {
        match test_property {
          'x' => result.x_min = test_comparison + 1,
          'm' => result.m_min = test_comparison + 1,
          'a' => result.a_min = test_comparison + 1,
          's' => result.s_min = test_comparison + 1,
          _ => panic!("unexpected test property: {}", test_property),
        };
      },
      '<' => {
        match test_property {
          'x' => result.x_max = test_comparison - 1,
          'm' => result.m_max = test_comparison - 1,
          'a' => result.a_max = test_comparison - 1,
          's' => result.s_max = test_comparison - 1,
          _ => panic!("unexpected test property: {}", test_property),
        };
      },
      _ => panic!("unexpected test operation: {}", test_operation),
    }
    result
  }

  // copy of the above, but calculates the new region where the test *fails*
  fn reduce_for_test_fail(&self, test: &Test) -> Self {
    let mut result = self.to_owned();
    let Test { test_property, test_operation, test_comparison } = test;
    match test_operation {
      '>' => {
        match test_property {
          'x' => result.x_max = *test_comparison,
          'm' => result.m_max = *test_comparison,
          'a' => result.a_max = *test_comparison,
          's' => result.s_max = *test_comparison,
          _ => panic!("unexpected test property: {}", test_property),
        };
      },
      '<' => {
        match test_property {
          'x' => result.x_min = *test_comparison,
          'm' => result.m_min = *test_comparison,
          'a' => result.a_min = *test_comparison,
          's' => result.s_min = *test_comparison,
          _ => panic!("unexpected test property: {}", test_property),
        };
      },
      _ => panic!("unexpected test operation: {}", test_operation),
    }
    result
  }
}

// the main part of the solution is going to have to be a recursive function
fn get_accepted_volume(starting_volume: &FourDimensionalRegion, rules: &HashMap<String, Vec<Rule>>, current: &PartDestination, current_index: usize) -> u64 {
  // short-circuit if region is empty
  if starting_volume.is_empty() {
    return 0;
  }

  match current {
    PartDestination::Accepted => {
      // everything in this region gets accepted, so count all its volume
      return starting_volume.volume();
    },
    PartDestination::Rejected => {
      // it all gets rejected, so there's nothing to add
      return 0;
    }
    PartDestination::Rule(label) => {
      let ruleset = rules.get(label).unwrap();
      let rule_to_consider = ruleset[current_index].clone();
    
      match rule_to_consider.test {
        None => {
          // there's no test, so all of this volume gets sent to the new rule
          return get_accepted_volume(starting_volume, rules, &rule_to_consider.destination, 0);
        },
        Some(test) => {
          let pass_volume = starting_volume.reduce_for_test(&test);
          let pass_result = get_accepted_volume(&pass_volume, rules, &rule_to_consider.destination, 0);
          let fail_volume = starting_volume.reduce_for_test_fail(&test);
          let fail_result = get_accepted_volume(&fail_volume, rules, current, current_index + 1);
          return pass_result + fail_result;
        },
      }
    }
  }
}

fn solve_part_2(data: &PuzzleData) -> u64 {
  get_accepted_volume(&FourDimensionalRegion::new(), &data.rules, &PartDestination::Rule(String::from("in")), 0)
}

pub fn part_2() -> u64 {
  let data = read_file();
  solve_part_2(&data)
}
