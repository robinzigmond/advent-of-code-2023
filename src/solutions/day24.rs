use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use divisors;

struct HailStone {
  x_position: i64,
  y_position: i64,
  z_position: i64,
  x_velocity: i64,
  y_velocity: i64,
  z_velocity: i64,
}

fn read_line(line: &str) -> HailStone {
  let parts: Vec<&str> = line.split(" @ ").collect();
  let position_parts: Vec<&str> = parts[0].split(",").collect();
  let velocity_parts: Vec<&str> = parts[1].split(",").collect();

  HailStone {
    x_position: position_parts[0].trim().parse().unwrap(),
    y_position: position_parts[1].trim().parse().unwrap(),
    z_position: position_parts[2].trim().parse().unwrap(),
    x_velocity: velocity_parts[0].trim().parse().unwrap(),
    y_velocity: velocity_parts[1].trim().parse().unwrap(),
    z_velocity: velocity_parts[2].trim().parse().unwrap(),
  }
}

fn read_file() -> Vec<HailStone> {
  let mut file = File::open("./input/input24.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  contents.lines().map(read_line).collect()
}

// as the puzzle says, at least for now we only need to consider intersections in the x, y plane
fn future_x_y_intersection_point(hailstone1: &HailStone, hailstone2: &HailStone) -> Option<(f64, f64)> {
  // if hailstone 1 has current position (px1, py1) and velocity (vx1, vy1), and the same for hailstone 2
  // with coordinates px2 etc, then the times a and b at which their paths cross are given by the
  // simultaneous equations:
  // px1 + a*vx1 = px2 + b*vx2
  // py1 + a*vy1 = py2 + b*vy2
  // One way to solve these for a and b is to eliminate b, by multiplying the top equation by vy2 and
  // the bottom by vx2, and subtracting them:
  // (px1*vy2 - py1*vx2) + a(vx1*vy2 - vy1*vx2) = px2*vy2 - py2*vx2
  // or:
  // a = (px2*vy2 + py1*vx2 - py2*vx2 - px1*vy2)/(vx1*vy2 - vy1*vx2)
  // from which we can solve for b as:
  // b = (px1*vy1 + py2*vx1 - py1*vx1 - px2*vy1)/(vx2*vy1 - vy2*vx1)
  // [this can be derived through algebraic manipulation if needed but it's easier just to note that if we
  // swapped round the two hailstones then a and b would trade places, as would all the **1 with the corresponding
  // **2]
  // The first thing we have to do is check that both a and b are positive - if not, the "collision point"
  // was in the past for one or both hailstones, which we are told to ignore.
  // Assuming a and b are OK, we can solve for x and y by substituting them in either side of the original 2
  // equations: eg px1 + a*vx1 is the x-corodinates, and py1 _ a*vy1 is the y-coordinate

  // note that it's possible for the paths to be parallel, in which case the following may crash on a division by 0.
  // - except it probably won't because of floating-point errors, but it will still give nonsensical results so we
  // need to test for it specifically. There will of course be no intersection in this case, so:

  if hailstone1.x_velocity * hailstone2.y_velocity == hailstone1.y_velocity * hailstone2.x_velocity {
    return None;
  }

  let time_1 = (hailstone2.x_position as f64 * hailstone2.y_velocity as f64
              + hailstone1.y_position as f64 * hailstone2.x_velocity as f64
              - hailstone2.y_position as f64 * hailstone2.x_velocity as f64
              - hailstone1.x_position as f64 * hailstone2.y_velocity as f64)
              /
              (hailstone1.x_velocity as f64 * hailstone2.y_velocity as f64
              - hailstone1.y_velocity as f64 * hailstone2.x_velocity as f64);

  let time_2 = (hailstone1.x_position as f64 * hailstone1.y_velocity as f64
              + hailstone2.y_position as f64 * hailstone1.x_velocity as f64
              - hailstone1.y_position as f64 * hailstone1.x_velocity as f64
              - hailstone2.x_position as f64 * hailstone1.y_velocity as f64)
              /
              (hailstone2.x_velocity as f64 * hailstone1.y_velocity as f64
              - hailstone2.y_velocity as f64 * hailstone1.x_velocity as f64);

  if time_1 < 0.0 || time_2 < 0.0 {
    return None;
  }

  let x = hailstone1.x_position as f64 + time_1 * hailstone1.x_velocity as f64;
  let y = hailstone1.y_position as f64 + time_1 * hailstone1.y_velocity as f64;

  Some((x, y))
}

fn solve_part_1(hailstones: &Vec<HailStone>) -> u32 {
  let min_value = 200000000000000f64;
  let max_value = 400000000000000f64;
  let mut intersections_to_count = 0;

  for i in 0..hailstones.len() {
    for j in (i + 1)..hailstones.len() {
      if let Some((x, y)) = future_x_y_intersection_point(&hailstones[i], &hailstones[j]) {
        if x >= min_value
        && x <= max_value
        && y >= min_value
        && y <= max_value {
          intersections_to_count += 1;
        }
      }
    }
  }

  intersections_to_count
}

pub fn part_1() -> u32 {
  let hailstones = read_file();
  solve_part_1(&hailstones)
}

fn solve_part_2(hailstones: &Vec<HailStone>) -> i64 {
  // as above, we denote the current position hailstone #n as (pxn, pyn, pzn), and the velocity as
  // (vxn, vyn, vzn).
  // Let us call the hypothetical start position of our rock (px0, py0, pz0) and its velocity (vx0, vy0, vz0).
  // Then it collides with hailstone #n at a hypothetical time tn which satisfies the 3 simultaneous equations:
  // p(x/y/z)n + tn*v(x/y/z)n = p(x/y/z)0 + tn*v(x/y/z)0
  // Further more we know that each of the 7 unknowns (tn, the 3 v(x/y/z)m and the 3 p(x/y/z)n) are integers,
  // and the tn are all positive.

  // Rewrite the above as:
  // tn(v(x/y/z)n - v(x/y/z)0) = p(x/y/z)0 - p(x/y/z)n
  // So we need to choose p(x/y/z)0 and v(x/y/z)0 such that, for each n:
  // v(x/y/z)n - v(x/y/z)0 divides p(x/y/z)0 - p(x/y/z)n exactly,
  // and further that the ratio is the same (for a given n) whichever of x/y/z is chosen.

  // This allows us to try a reasonable trial and error approach that quickly narrows down the possibilities. This is made
  // much easier by the fact that - in the real data, as well as the test data - there are repeating velocities
  // among the hailstones, in each of the x, y and z directions.
  // This is important, because for each hailstone and each of the directions, we get:
  // v(x/y/z)n - v(x/y/z)0 divides p(x/y/z)0 - p(x/y/z)n
  // and if v(x/y/z)m = v(x/y/z)n for some known m and n and particular one of x/y/z (as will be the case), then the
  // common quantity on the left divides both p(x/y/z)0 - p(x/y/z)n and p(x/y/z)0 - p(x/y/z)m, so it divides their
  // difference: p(x/y/z)n - p(x/y/z)m. Since this is a known quantity, we an easily check all its factors, and then
  // v(x/y/z)0 must be the result of subtracting one of these factors from v(x/y/z)n, and the same for v(x/y/z)m.
  // This may already give us a big restriction on what v(x/y/z)0 could be - and any remaining possibilities can be
  // checked against the individual equations for the tn's to either rule them out, or find the solution for the p(x/y/z)0s
  // that we need!

  // First let's get divisibility information for each velocity co-ordinate.
  let mut x_pairs: HashMap<i64, HashSet<i64>> = HashMap::new();
  let mut y_pairs: HashMap<i64, HashSet<i64>> = HashMap::new();
  let mut z_pairs: HashMap<i64, HashSet<i64>> = HashMap::new();
  for hailstone in hailstones {
    x_pairs.entry(hailstone.x_velocity)
           .and_modify(|set| {
            set.insert(hailstone.x_position);
           })
           .or_insert({
            let mut new_set = HashSet::new();
            new_set.insert(hailstone.x_position);
            new_set
          });

    y_pairs.entry(hailstone.y_velocity)
          .and_modify(|set| {
           set.insert(hailstone.y_position);
          })
          .or_insert({
           let mut new_set = HashSet::new();
           new_set.insert(hailstone.y_position);
           new_set
         });

    z_pairs.entry(hailstone.z_velocity)
         .and_modify(|set| {
          set.insert(hailstone.z_position);
         })
         .or_insert({
          let mut new_set = HashSet::new();
          new_set.insert(hailstone.z_position);
          new_set
        });
  }
  // want to remove any single positions so we can see clearly what repeats we have
  x_pairs.retain(|_, positions| positions.len() > 1);
  y_pairs.retain(|_, positions| positions.len() > 1);
  z_pairs.retain(|_, positions| positions.len() > 1);

  // use these to get all divisors of differences
  let mut possible_x_velocities = HashSet::new();
  let mut possible_y_velocities = HashSet::new();
  let mut possible_z_velocities = HashSet::new();
  for (velocity, positions) in x_pairs {
    let mut possibilities = vec![];
    // double-loop through the positions to get all pairs
    for value1 in positions.clone() {
      for &value2 in positions.difference(&HashSet::from([value1])) {
        // need to ensure the difference is positive for the get_divisors function to work
        let difference = if value2 > value1 { (value2 - value1) as u64 } else { (value1 - value2) as u64 };
        let divisors = divisors::get_divisors(difference);
        possibilities.append(&mut divisors.iter().map(|&n| n as i64 + velocity).collect());
        // add the negative divisors back in
        possibilities.append(&mut divisors.iter().map(|&n| velocity -(n as i64)).collect());
        // we also need to add the position itself (and its negative) in, as the get_divisors function only returns
        // proper divisors. Same with +1 and -1.
        possibilities.append(&mut vec![difference as i64 + velocity, velocity - (difference as i64), velocity + 1, velocity - 1]);
      }
    }
    // if this is the first time through, add all the possibilities in:
    if possible_x_velocities.is_empty() {
      possible_x_velocities = HashSet::from_iter(possibilities.into_iter());
    } else {
      possible_x_velocities = possible_x_velocities.intersection(&HashSet::from_iter(possibilities.into_iter())).map(|&n| n).collect();
    }
  }
  for (velocity, positions) in y_pairs {
    let mut possibilities = vec![];
    // double-loop through the positions to get all pairs
    for value1 in positions.clone() {
      for &value2 in positions.difference(&HashSet::from([value1])) {
        // need to ensure the difference is positive for the get_divisors function to work
        let difference = if value2 > value1 { (value2 - value1) as u64 } else { (value1 - value2) as u64 };
        let divisors = divisors::get_divisors(difference);
        possibilities.append(&mut divisors.iter().map(|&n| n as i64 + velocity).collect());
        // add the negative divisors back in
        possibilities.append(&mut divisors.iter().map(|&n| velocity -(n as i64)).collect());
        // we also need to add the position itself (and its negative) in, as the get_divisors function only returns
        // proper divisors. Same with +1 and -1.
        possibilities.append(&mut vec![difference as i64 + velocity, velocity - (difference as i64), velocity + 1, velocity - 1]);
      }
    }
    // if this is the first time through, add all the possibilities in:
    if possible_y_velocities.is_empty() {
      possible_y_velocities = HashSet::from_iter(possibilities.into_iter());
    } else {
      possible_y_velocities = possible_y_velocities.intersection(&HashSet::from_iter(possibilities.into_iter())).map(|&n| n).collect();
    }
  }
  for (velocity, positions) in z_pairs {
    let mut possibilities = vec![];
    // double-loop through the positions to get all pairs
    for value1 in positions.clone() {
      for &value2 in positions.difference(&HashSet::from([value1])) {
        // need to ensure the difference is positive for the get_divisors function to work
        let difference = if value2 > value1 { (value2 - value1) as u64 } else { (value1 - value2) as u64 };
        let divisors = divisors::get_divisors(difference);
        possibilities.append(&mut divisors.iter().map(|&n| n as i64 + velocity).collect());
        // add the negative divisors back in
        possibilities.append(&mut divisors.iter().map(|&n| velocity -(n as i64)).collect());
        // we also need to add the position itself (and its negative) in, as the get_divisors function only returns
        // proper divisors. Same with +1 and -1.
        possibilities.append(&mut vec![difference as i64 + velocity, velocity - (difference as i64), velocity + 1, velocity - 1]);
      }
    }
    // if this is the first time through, add all the possibilities in:
    if possible_z_velocities.is_empty() {
      possible_z_velocities = HashSet::from_iter(possibilities.into_iter());
    } else {
      possible_z_velocities = possible_z_velocities.intersection(&HashSet::from_iter(possibilities.into_iter())).map(|&n| n).collect();
    }
  }

  // it turns out that for the real data there is exactly one possibility for each co-ordinate after doing these.
  // This is not the case for the test data (where that only applies to the y co-ordinate!).
  // We will use this to greatly simplify the rest of the process.
  if possible_x_velocities.len() != 1 {
    panic!("not just 1 possible x velocity");
  }
  let x_velocity = *possible_x_velocities.iter().next().unwrap();
  if possible_y_velocities.len() != 1 {
    panic!("not just 1 possible y velocity");
  }
  let y_velocity = *possible_y_velocities.iter().next().unwrap();
  if possible_z_velocities.len() != 1 {
    panic!("not just 1 possible z velocity");
  }
  let z_velocity = *possible_z_velocities.iter().next().unwrap();

  // now we have to work out the position for each of these velocities.
  // Because we know the velocities, the necessary equations for a single hailstone contain just 4 unknowns
  // (the time and the x/y/z positions). Add in a second hailstone and we have 6 equations but only 5 unknowns
  // (the second time of intersection being the only new unknown). So we can easily solve this with linear
  // algebra.
  // I'll spare the details of the algebra in this comment but leave the computations to work out the answer:
  let hailstone_1_time_coeff_x = hailstones[0].x_velocity - x_velocity;
  let hailstone_1_const_x = hailstones[0].x_position;
  let hailstone_1_time_coeff_y = hailstones[0].y_velocity - y_velocity;
  let hailstone_1_const_y = hailstones[0].y_position;
  let hailstone_1_time_coeff_z = hailstones[0].z_velocity - z_velocity;
  let hailstone_1_const_z = hailstones[0].z_position;

  let hailstone_2_time_coeff_x = hailstones[1].x_velocity - x_velocity;
  let hailstone_2_const_x = hailstones[1].x_position;
  let hailstone_2_time_coeff_y = hailstones[1].y_velocity - y_velocity;
  let hailstone_2_const_y = hailstones[1].y_position;
 
  let hailstone_1_time_numerator = hailstone_2_time_coeff_y * (hailstone_2_const_x - hailstone_1_const_x)
                                  + hailstone_2_time_coeff_x * (hailstone_1_const_y - hailstone_2_const_y);
  
  let hailstone_1_time_denominator = hailstone_1_time_coeff_x * hailstone_2_time_coeff_y - hailstone_1_time_coeff_y * hailstone_2_time_coeff_x;
  assert!(hailstone_1_time_numerator % hailstone_1_time_denominator == 0);
  let hailstone_1_time = hailstone_1_time_numerator / hailstone_1_time_denominator;

  let x_position = hailstone_1_time_coeff_x * hailstone_1_time + hailstone_1_const_x;
  let y_position = hailstone_1_time_coeff_y * hailstone_1_time + hailstone_1_const_y;
  let z_position = hailstone_1_time_coeff_z * hailstone_1_time + hailstone_1_const_z;

  x_position + y_position + z_position
}

pub fn part_2() -> i64 {
  let hailstones = read_file();
  solve_part_2(&hailstones)
}
