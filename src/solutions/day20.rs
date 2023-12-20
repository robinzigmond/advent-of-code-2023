use num::integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy)]
enum Pulse {
  Low,
  High,
}

#[derive(Clone)]
enum FlipFlopState {
  Off,
  On,
}

#[derive(Clone)]
struct FlipFlopModule {
  state: FlipFlopState,
  outputs: Vec<String>,
}

#[derive(Clone)]
struct ConjunctionModule {
  state: HashMap<String, Pulse>,
  outputs: Vec<String>,
}

#[derive(Clone)]
struct BroadcastModule {
  outputs: Vec<String>,
}

#[derive(Clone)]
enum Module {
  FlipFlop(FlipFlopModule),
  Conjunction(ConjunctionModule),
  Broadcast(BroadcastModule),
}

impl Module {
  // simple utility to get a list of outputs, without having to worry about the type of the module
  fn get_outputs(&self) -> &Vec<String> {
    match self {
      Module::FlipFlop(module) => &module.outputs,
      Module::Conjunction(module) => &module.outputs,
      Module::Broadcast(module) => &module.outputs,
    }
  }

  // main function to process an input. Outputs a list of all pulses output, tagged with the names of
  // both the sender (this module) and the receiver
  fn process_input(&mut self, own_name: String, input: Pulse, sender: String) -> Vec<(String, String, Pulse)> {
    match self {
      Module::Broadcast(module) => {
        return module.outputs.clone().into_iter().map(|output| (own_name.clone(), output, input)).collect();
      },
      Module::FlipFlop(module) => {
        if let Pulse::Low = input {
          match module.state {
            FlipFlopState::Off => {
              module.state = FlipFlopState::On;
              return module.outputs.clone().into_iter().map(|output| (own_name.clone(), output, Pulse::High)).collect();
            },
            FlipFlopState::On => {
              module.state = FlipFlopState::Off;
              return module.outputs.clone().into_iter().map(|output| (own_name.clone(), output, Pulse::Low)).collect();
            },
          }
        } else {
          return vec![];
        }
      },
      Module::Conjunction(module) => {
        module.state.insert(sender, input);
        let output_pulse = if module.state.values().all(|pulse| match pulse {
          Pulse::High => true,
          Pulse::Low => false,
        }) { Pulse::Low } else { Pulse::High };
        return module.outputs.clone().into_iter().map(|output| (own_name.clone(), output, output_pulse)).collect();
      },
    }
  }
}

fn read_file() -> HashMap<String, Module> {
  let mut file = File::open("./input/input20.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut modules = HashMap::new();

  // do an initial pass through, initialising all Conjunction modules as having empty inputs
  for line in contents.lines() {
    let parts: Vec<&str> = line.split(" -> ").collect();
    let name_info = parts[0];
    let outputs = parts[1].split(", ").map(|s| s.to_owned()).collect();

    if name_info == "broadcaster" {
      modules.insert(String::from("broadcaster"), Module::Broadcast(BroadcastModule {
        outputs
      }));
    } else {
      let as_vec: Vec<char> = name_info.chars().collect();
      let (first_char, name) = as_vec.split_at(1);
      let first_char = first_char[0];
      let name: String = name.iter().collect();

      match first_char {
        '%' => {
          modules.insert(name, Module::FlipFlop(FlipFlopModule { state: FlipFlopState::Off, outputs }));
        },
        '&' => {
          modules.insert(name, Module::Conjunction(ConjunctionModule { state: HashMap::new(), outputs }));
        },
        _ => panic!("unexpected first character: {}", first_char),
      }
    }
  }

  // now go through all modules again and add the initial default low pulses to memory
  // of the Broadcast modules for each of their inputs
  let mut conjunction_inputs: HashMap<String, Vec<String>> = HashMap::new();
  for (name, module) in &modules {
    let outputs = module.get_outputs();
    for output in outputs {
      let output_module = modules.get(output);
      if let Some(Module::Conjunction(_)) = output_module {
        let previous_inputs = conjunction_inputs.get_mut(output);
        match previous_inputs {
          Some(previous) => {
            previous.push(name.to_owned());
          },
          None => {
            conjunction_inputs.insert(output.to_owned(), vec![]);
          },
        }
      }
    }
  }
  for (output, inputs) in conjunction_inputs {
    for input in inputs {
      match modules.get_mut(&output).unwrap() {
        Module::Conjunction(module) => {
          module.state.insert(input, Pulse::Low);
        },
        _ => panic!("we know this must be a conjunction module..."),
      }
    }
  }

  modules
}

// simulates a single button press (which sends a "low" input to the broadcast module).
// Returns the total number of both low and high pulses sent (as that's what we need for part 1).
// For part 2, it takes an optional argument of a particular module name to watch for when a low pulse
// is sent to it, and returns a boolean indicating if it ever was.
fn press_button(modules: &mut HashMap<String, Module>, module_to_watch: Option<&str>) -> (u64, u64, bool) {
  let mut pulse_queue = VecDeque::new();
  pulse_queue.push_back((String::new(), String::from("broadcaster"), Pulse::Low));
  let mut low_count = 1; // must count 1 for the initial button press!
  let mut high_count = 0;

  while !pulse_queue.is_empty() {
    let (sender, destination, pulse) = pulse_queue.pop_front().unwrap();
    // ignore any "output" modules which don't get processed further
    let processing_module = match modules.get_mut(&destination) {
      Some(module) => module,
      None => continue,
    };
    let new_outputs = processing_module.process_input(destination, pulse, sender);

    if let Some(desired_module) = module_to_watch {
      if let Some(_) = new_outputs.iter().find(|(_, output_module, pulse)| match pulse {
        Pulse::High => false,
        Pulse::Low => output_module == desired_module,
      }) {
        return (low_count, high_count, true);
      }
    }

    for output in new_outputs {
      // count the pulses, then add to the queue
      match output.2 {
        Pulse::Low => low_count += 1,
        Pulse::High => high_count += 1,
      };
      pulse_queue.push_back(output);
    }
  }

  (low_count, high_count, false)
}

fn solve_part_1(modules: &mut HashMap<String, Module>) -> u64 {
  let mut low_total = 0;
  let mut high_total = 0;

  for _ in 0..1000 {
    let (low, high, _) = press_button(modules, None);
    low_total += low;
    high_total += high;
  }

  low_total * high_total
}

pub fn part_1() -> u64 {
  let mut modules = read_file();
  solve_part_1(&mut modules)
}

// for part 2, we just have to notice that there are 4 different, independent parts of the set of modules.
// Each culminates in a Conjunction module with many inputs. These 4 "final" conjunction modules then each go
// through another conjunction with it as the only input (so negating), before they combine as inputs to
// to a "combiner" module, mf, which outputs to rx.
// As a result, to get a low pulse into rx,  we need a high pulse to each of those inputs to mf, which mean
// the "final" modules mentioned at the start each needs to output a low pulse.

// the below function finds the earliest number of button presses to preduce a low pulse for a given
// module

fn wait_for_low_pulse(modules: &mut HashMap<String, Module>, module_name: String) -> u64 {
  let mut button_count = 0;

  loop {
    button_count += 1;
    let (_, _, result) = press_button(modules, Some(&module_name));
    if result {
      return button_count;
    }
  }
}

// a recursive function that traverses the module tree and breaks it up into "subsystems" of independent modules,
// returning the final one *before* the one (which turns out to be mf) that connects them all together.
fn get_subtrees(modules: &HashMap<String, Module>, current_module: String, parent: Option<String>, so_far: &mut Vec<String>) -> Option<String> {
  let mut result = None;
  // cheat by hardcoding the module we need to end at
  if &current_module == "mf" {
    return parent;
  }
  if so_far.contains(&current_module) {
    return None;
  }
  so_far.push(current_module.clone());
  let outputs = modules.get(&current_module).unwrap().get_outputs();
  for output in outputs {
    if let Some(found) = get_subtrees(modules, output.clone(), Some(current_module.clone()), so_far) {
      result = Some(found);
    }
  }
  return result;
}

fn solve_part_2(modules: &mut HashMap<String, Module>) -> u64 {
  let mut final_modules = vec![];
  let start_modules = modules.get("broadcaster").unwrap().get_outputs();
  for start in start_modules {
    let mut current_subsystem = vec![];
    let final_module = get_subtrees(modules, String::from(start), Some(String::from("broadcaster")), &mut current_subsystem).unwrap();
    // also need a fresh copy of the modules (in original state) for each time we run through it!
    final_modules.push((final_module, modules.clone()));
  }

  // iTo solve this in full generality we would need to also know the length it takes for each subsystem to cycle
  // back to the starting state - which may well be more than the wait_time. But having actually implemented this
  // at an earlier stage, it turns out that it's always equal to the wait_time, which makes it much easier to work out
  // - so we just make that assumption below.

  let mut result = 1;

  for (final_module, mut clean_modules) in final_modules {
    let wait_time = wait_for_low_pulse(&mut clean_modules, final_module.clone());
    result = lcm(result, wait_time);
  }
  result
}

pub fn part_2() -> u64 {
  let mut modules = read_file();
  solve_part_2(&mut modules)
}
