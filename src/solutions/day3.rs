use std::fs::File;
use std::io::prelude::*;

enum EngineEntity {
  PartNumber(u32),
  Symbol(char),
}

struct PositionedEntity {
  entity: EngineEntity,
  // of course the positions cannot be negative, but when computing possible neighbours it's easier to
  // allow -1 values
  position: (isize, isize),
}

fn read_file() -> Vec<PositionedEntity> {
  let mut file = File::open("./input/input3.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  
  let mut engine = vec![];
  for (row_index, line) in contents.lines().enumerate() {
    let mut number_so_far: Option<u32> = None;
    for (column_index, char) in line.chars().enumerate() {
      let mut is_num_finished = column_index == line.len() - 1;
      let mut finished_before_end = false;
      match char.to_digit(10) {
        Some(digit) => {
          match number_so_far {
            Some(num) => number_so_far = Some(num * 10 + digit),
            None => number_so_far = Some(digit),
          }
        }
        None => {
          is_num_finished = number_so_far.is_some();
          finished_before_end = true;
          if char != '.' {
            let entity = EngineEntity::Symbol(char);
            engine.push(PositionedEntity { entity, position: (row_index as isize, column_index as isize) })
          }
        }
      }
      if is_num_finished {
        let part_number = number_so_far.unwrap();
        let part = EngineEntity::PartNumber(part_number);
        let index_when_finished = if finished_before_end { column_index } else { column_index + 1 };
        let position = (row_index as isize, (index_when_finished - part_number.to_string().len()) as isize);
        engine.push(PositionedEntity {
          entity: part,
          position
        });
        number_so_far = None;
      }
    }
  }
  engine
}

fn solve_part_1(engine: Vec<PositionedEntity>) -> u32 {
  // this is going to be O(n^2), where n is the total number of "entities". Hopefully doesn't matter when
  // it's only day 3!
  let mut part_sum = 0;
  for entity in &engine {
    if let EngineEntity::PartNumber(num) = entity.entity {
      let mut symbol_positions = vec![];
      let (row, col) = entity.position;
      for i in 0..num.to_string().len() {
        let i = i as isize;
        symbol_positions.append(&mut vec![
          (row - 1, col + i - 1),
          (row - 1, col + i),
          (row - 1, col + i + 1),
          (row, col + i - 1),
          (row, col + i + 1),
          (row + 1, col + i - 1),
          (row + 1, col + i),
          (row + 1, col + i + 1)
        ]);
      }
      let mut is_genuine_part = false;
      for other_entity in &engine {
        if let EngineEntity::Symbol(_) = other_entity.entity {
          if symbol_positions.contains(&other_entity.position) {
            is_genuine_part = true;
            break;
          }
        }
      }
      if is_genuine_part {
        part_sum += num;
      }
    }
  }
  part_sum
}

pub fn part_1() -> u32 {
  let engine = read_file();
  solve_part_1(engine)
}

fn solve_part_2(engine: Vec<PositionedEntity>) -> u32 {
  let mut ratio_sum = 0;
  for entity in &engine {
    if let EngineEntity::Symbol('*') = entity.entity {
      let mut ratio = 1;
      let mut touching_nums = 0;
      for other_entity in &engine {
        if let EngineEntity::PartNumber(n) = other_entity.entity {
          let (y, x) = other_entity.position;
          let num_length = n.to_string().len() as isize;
          let (y0, x0) = entity.position;
          if x >= x0 - num_length && x <= x0 + 1 && (y -  y0).abs() <= 1 {
            touching_nums += 1;
            ratio *= n;
          }
        }
      }
      if touching_nums == 2 {
        ratio_sum += ratio;
      }
    }
  }
  ratio_sum
}

pub fn part_2() -> u32 {
  let engine = read_file();
  solve_part_2(engine)
}
