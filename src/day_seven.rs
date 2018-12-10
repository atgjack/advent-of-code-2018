use std::collections::HashMap;

pub fn find_instruction_order(file: &'static str) -> String {

  let mut deps = parse_instructions(file);
  let mut result: Vec<char> = Vec::new();

  while deps.keys().len() > 0 {
    let mut next: Vec<char> = deps
      .iter()
      .filter(|(_,v)| v.is_empty())
      .map(|(k, _)| *k)
      .collect();

    next.sort();

    let instruction = next.iter().next().unwrap();

    result.push(*instruction);
    deps.remove(instruction);
    complete_dependency(&mut deps, *instruction);
  }

  result.iter().collect()
}

pub fn find_time_with_workers(file: &'static str, num_workers: usize, time_adder: usize) -> usize {

  let mut deps = parse_instructions(file);
  let mut tick: usize = 0;
  let mut workers: Vec<Option<(char, usize)>> = vec![None; num_workers];

  loop {
    if workers.iter().filter(|w| w.is_some()).count() == 0 && deps.keys().len() == 0 {
      break;
    }

    let mut next: Vec<char> = deps
      .iter()
      .filter(|(_,v)| v.is_empty())
      .map(|(k, _)| *k)
      .filter(|c| {
        workers
          .iter()
          .filter_map(|o| *o)
          .filter(|(wc, _)| c == wc)
          .count() == 0
      })
      .collect();

    workers = workers.iter()
      .map(|w| {
        if let Some((c, t)) = w {
          if *t <= 1 {
            deps.remove(c);
            complete_dependency(&mut deps, *c);
            None
          } else {
            Some((*c, t - 1))
          }
        } else {
          None
        }
      })
      .map(|w| {
        if w.is_some() || next.is_empty() { 
          w
        } else {
          let letter = next.pop().unwrap();
          let count = (letter as u8 - b'A') as usize;
          Some((letter, count + time_adder))
        }
      })
      .collect();

    tick += 1;
  }

  tick
}

#[inline]
fn parse_instructions(file: &'static str) -> HashMap<char, Vec<char>> {
  let mut instructions = file.lines()
    .map(|l| (l[5..6].chars().next().unwrap(), l[36..37].chars().next().unwrap()))
    .fold(HashMap::new(), |mut map, (d, c)| {
      map.entry(d).or_insert_with(Vec::new);
      map.entry(c).or_insert_with(Vec::new).push(d);
      map
    });

  sort_dependencies(&mut instructions);

  instructions
}

#[inline]
fn complete_dependency(map: &mut HashMap<char, Vec<char>>, dep: char) {
  map.values_mut()
    .for_each(|v| {
      v.remove_item(&dep);
    });
}

#[inline]
fn sort_dependencies(map: &mut HashMap<char, Vec<char>>) {
  map.values_mut()
    .for_each(|v| {
      v.sort();
    });
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_find_instruction_order() {
    let test_file = include_str!("test_inputs/day_seven/simple_instructions.txt");
    let actual = find_instruction_order(test_file);

    assert_eq!(actual, "CABDFE");
  }

    #[test]
  fn can_find_time_with_workers() {
    let test_file = include_str!("test_inputs/day_seven/simple_instructions.txt");
    let actual = find_time_with_workers(test_file, 2, 0);

    assert_eq!(actual, 15);
  }
}